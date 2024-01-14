use core::f32::consts::PI;
use cpal::{
	platform::Stream,
	traits::{DeviceTrait, HostTrait, StreamTrait},
	FromSample, Sample,
};
use crossterm::{cursor, terminal, ExecutableCommand};
use promptly::prompt_default;
use realfft::{num_complex::Complex, RealFftPlanner, RealToComplex};
use std::collections::VecDeque;
use std::error::Error;
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};

// mean and var: mean and var over 70 samples
pub const NB_AUDIO_CHANNELS: usize = 3;
const STAT_WINDOW_SIZE: u32 = 70;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Data {
	pub gain: [f32; NB_AUDIO_CHANNELS],
	_offset: f32,
}

impl Data {
	pub fn new() -> Data {
		Data {
			gain: [0.0; NB_AUDIO_CHANNELS],
			_offset: 0.0,
		}
	}
}

struct Buffer {
	input: Vec<f32>,
	output: Vec<Complex<f32>>,
	scratch: Vec<Complex<f32>>,
	window: Vec<f32>,
	pos: usize,
	len: usize,
	r2c: Arc<dyn RealToComplex<f32>>,
	mean: Vec<f32>,
	var: Vec<f32>,
	count: u64,
	index_limits: Vec<usize>,
	stat_window: Vec<VecDeque<f32>>,
	stat_window_size: u32,
}

pub fn init(
	chunck_size: u32,
	min_freq: u32,
	max_freq: u32,
) -> Result<(Arc<Mutex<Data>>, Stream), Box<dyn Error>> {
	/*
	// For debugging
	let hosts = cpal::available_hosts();
	for h in hosts {
		println!("{:?}", h);
	}
	*/

	let host = cpal::default_host();
	let devices: Vec<_> = host.input_devices()?.collect();

	for (i, d) in devices.iter().enumerate() {
		println!("[DEVICE {}] {}", i, d.name()?);
	}

	let device_id: usize = prompt_default("Select Device Id", 0)?;

	let device = &devices[device_id];

	let config = device.default_input_config()?;

	println!("[DEFAULT AUDIO CONFIG] {:?}", config);

	let stat_window_size = STAT_WINDOW_SIZE;

	let mut real_planner = RealFftPlanner::<f32>::new();
	let r2c = real_planner.plan_fft_forward(chunck_size as usize);
	let input = r2c.make_input_vec();
	let output = r2c.make_output_vec();
	let scratch = r2c.make_scratch_vec();
	let stat_window = vec![VecDeque::new(); NB_AUDIO_CHANNELS];
	let hanning_window = (0..input.len())
		.map(|i| 0.5 * (1.0 - ((2.0 * PI * i as f32) / (input.len() - 1) as f32).cos()))
		.collect();

	let mut buffer = Buffer {
		input,
		output,
		scratch,
		len: chunck_size as usize,
		pos: 0,
		r2c,
		mean: vec![0.0; NB_AUDIO_CHANNELS],
		var: vec![0.0; NB_AUDIO_CHANNELS],
		count: 0,
		window: hanning_window,
		index_limits: calculate_channel_index(
			min_freq,
			max_freq,
			NB_AUDIO_CHANNELS as u32,
			config.sample_rate().0,
			chunck_size,
		),
		stat_window,
		stat_window_size,
	};

	let err_fn = move |err| {
		eprintln!("an error occurred on stream: {}", err);
	};

	let audio_data = Data {
		gain: [0.0; NB_AUDIO_CHANNELS],
		_offset: 0.0,
	};
	let audio_data_arc = Arc::new(Mutex::new(audio_data));
	let audio_data_arc1 = audio_data_arc.clone();

	let stream = match config.sample_format() {
		cpal::SampleFormat::I8 => device.build_input_stream(
			&config.into(),
			move |data, _: &_| handle_input::<i8>(data, &mut buffer, &audio_data_arc1),
			err_fn,
			None,
		)?,
		cpal::SampleFormat::I16 => device.build_input_stream(
			&config.into(),
			move |data, _: &_| handle_input::<i16>(data, &mut buffer, &audio_data_arc1),
			err_fn,
			None,
		)?,
		cpal::SampleFormat::I32 => device.build_input_stream(
			&config.into(),
			move |data, _: &_| handle_input::<i32>(data, &mut buffer, &audio_data_arc1),
			err_fn,
			None,
		)?,
		cpal::SampleFormat::F32 => device.build_input_stream(
			&config.into(),
			move |data, _: &_| handle_input::<f32>(data, &mut buffer, &audio_data_arc1),
			err_fn,
			None,
		)?,
		_ => return Err(Box::from("Unsupported sample format")),
	};

	for _ in 0..NB_AUDIO_CHANNELS {
		println!("");
	}

	stream.play()?;
	Ok((audio_data_arc, stream))
}

fn calculate_channel_index(
	min_freq: u32,
	max_freq: u32,
	nb_channels: u32,
	sample_rate: u32,
	chunck_size: u32,
) -> Vec<usize> {
	let nb_octaves = (max_freq as f32 / min_freq as f32).log2();
	let nb_octaves_per_channel = nb_octaves / nb_channels as f32;
	let index_limits = (0..nb_channels + 1)
		.map(|i| {
			(min_freq * 2_f32.powf(nb_octaves_per_channel * i as f32) as u32 * chunck_size
				/ sample_rate) as usize
		})
		.collect();

	index_limits
}

fn handle_input<T>(input: &[T], buffer: &mut Buffer, audio_data: &Arc<Mutex<Data>>)
where
	T: Sample,
	f32: FromSample<T>,
{
	// every 2 because stereo
	for &sample in input.iter().step_by(2) {
		let pos = buffer.pos;
		// apply window
		buffer.input[pos] = f32::from_sample(sample) * buffer.window[pos];
		buffer.pos = pos + 1;
		if buffer.pos == buffer.len {
			buffer.pos = 0;
			buffer.count += 1;
			buffer
				.r2c
				.process_with_scratch(&mut buffer.input, &mut buffer.output, &mut buffer.scratch)
				.unwrap();

			// compute levels
			let levels: Vec<_> = (0..NB_AUDIO_CHANNELS)
				.map(|x| {
					(buffer.index_limits[x] + 1..buffer.index_limits[x + 1])
						.fold(0.0, |acc, i| acc + buffer.output[i].norm())
				})
				.collect();

			// update mean, sd and stat_window
			let tmp_inv = 1.0 / (buffer.stat_window_size) as f32;

			// Initialization
			if buffer.count <= buffer.stat_window_size as u64 {
				for i in 0..NB_AUDIO_CHANNELS {
					buffer.stat_window[i].push_front(levels[i]);
					buffer.mean[i] += tmp_inv * levels[i];
					buffer.var[i] += tmp_inv * levels[i].powi(2);
					if buffer.count == buffer.stat_window_size as u64 {
						buffer.var[i] -= buffer.mean[i].powi(2);
					}
				}
			} else {
				for i in 0..NB_AUDIO_CHANNELS {
					let last_val = buffer.stat_window[i].pop_back().unwrap();
					buffer.stat_window[i].push_front(levels[i]);

					let cur_mean = buffer.mean[i];

					buffer.mean[i] = cur_mean + tmp_inv * (levels[i] - last_val);
					buffer.var[i] = buffer.var[i]
						+ tmp_inv * (levels[i].powi(2) - last_val.powi(2))
						+ (cur_mean.powi(2) - buffer.mean[i].powi(2));

					if buffer.var[i] < 0.0 {
						buffer.var[i] = 0.0;
					}
				}
			}

			//check if there is at least one value over the threshold
			let threshold = 5.0;
			let mut over = false;
			for x in &buffer.output {
				if x.norm() > threshold {
					over = true;
					break;
				}
			}

			let mut gain = [f32::MIN; NB_AUDIO_CHANNELS];
			if over {
				for i in 0..NB_AUDIO_CHANNELS {
					gain[i] = (levels[i] - buffer.mean[i]) / buffer.var[i].sqrt();
				}
			}

			let mut stdout = stdout();
			stdout.execute(cursor::MoveUp(gain.len() as u16)).unwrap();
			stdout
				.execute(terminal::Clear(terminal::ClearType::FromCursorDown))
				.unwrap();
			for (i, g) in gain.iter().enumerate() {
				writeln!(stdout, "[{}]: {}", i, g).unwrap();
			}
			let mut audio_data = audio_data.lock().unwrap();
			audio_data.gain = gain;
			return;
		}
	}
}
