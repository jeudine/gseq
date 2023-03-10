use cpal::{
	platform::Stream,
	traits::{DeviceTrait, HostTrait, StreamTrait},
	FromSample, Sample,
};

use core::f32::consts::PI;
use realfft::{num_complex::Complex, RealFftPlanner, RealToComplex};
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct FFT {
	stream: Stream,
	pub level: Arc<Mutex<Vec<Level>>>,
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
	count: u64,
	nb_channels: u32,
	index_limits: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct Level {
	pub val: f32,
	pub mean: f32,
}

impl FFT {
	pub fn init(
		chunck_size: u32,
		nb_channels: u32,
		min_freq: u32,
		max_freq: u32,
	) -> Result<FFT, Box<dyn Error>> {
		let host = cpal::default_host();
		let device = host
			.default_input_device()
			.expect("failed to find input device");

		println!("Input device: {}", device.name()?);

		let config = device
			.default_input_config()
			.expect("Failed to get default input config");
		println!("Default input config: {:?}", config);

		let mut real_planner = RealFftPlanner::<f32>::new();
		let r2c = real_planner.plan_fft_forward(chunck_size as usize);
		let input = r2c.make_input_vec();
		let output = r2c.make_output_vec();
		let scratch = r2c.make_scratch_vec();
		let mean = vec![0.0; nb_channels as usize];
		let var = vec![0.0; nb_channels as usize];
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
			mean,
			count: 0,
			window: hanning_window,
			nb_channels,
			index_limits: Self::calculate_channel_index(
				min_freq,
				max_freq,
				nb_channels,
				config.sample_rate().0,
				chunck_size,
			),
		};

		let err_fn = move |err| {
			eprintln!("an error occurred on stream: {}", err);
		};

		let gain = vec![
			Level {
				val: 0.0,
				mean: 0.0
			};
			nb_channels as usize
		];
		let level_arc = Arc::new(Mutex::new(gain));
		let level_arc2 = level_arc.clone();

		let stream = match config.sample_format() {
			cpal::SampleFormat::I8 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i8>(data, &mut buffer, &level_arc2),
				err_fn,
				None,
			)?,
			cpal::SampleFormat::I16 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i16>(data, &mut buffer, &level_arc2),
				err_fn,
				None,
			)?,
			cpal::SampleFormat::I32 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i32>(data, &mut buffer, &level_arc2),
				err_fn,
				None,
			)?,
			cpal::SampleFormat::F32 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<f32>(data, &mut buffer, &level_arc2),
				err_fn,
				None,
			)?,
			_ => return Err(Box::from("Unsupported sample format")),
		};

		stream.play()?;

		Ok(Self {
			stream,
			level: level_arc,
		})
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

		for i in &index_limits {
			println!(
				"index: {}, freq: {}",
				i,
				i * sample_rate as usize / chunck_size as usize
			);
		}
		index_limits
	}
}

// TODO: maybe devide size of the buffer by 2
fn write_input_data<T>(input: &[T], buffer: &mut Buffer, level: &Arc<Mutex<Vec<Level>>>)
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
			/*
			for (i, el) in buffer.output.iter().enumerate() {
				println!("{}: {}", i, el.norm());
			}
			*/

			//check if there is at least one value over the threshold
			let threshold = 10.0;
			let mut over = false;
			for x in &buffer.output {
				if x.norm() > threshold {
					over = true;
					break;
				}
			}

			if !over {
				let new_level: Vec<_> = (0..buffer.nb_channels as usize)
					.map(|i| Level {
						val: 0.0,
						mean: buffer.mean[i],
					})
					.collect();
				let mut level = level.lock().unwrap();
				*level = new_level;
				return;
			}

			// compute levels
			let levels: Vec<_> = (0..buffer.nb_channels as usize)
				.map(|x| {
					(buffer.index_limits[x] + 1..buffer.index_limits[x + 1])
						.fold(0.0, |acc, i| acc + buffer.output[i].norm())
				})
				.collect();

			let stat_mem = 100;

			// update mean
			if buffer.count == 1 {
				for i in 0..buffer.nb_channels as usize {
					buffer.mean[i] = levels[i];
				}
			} else if buffer.count < stat_mem {
				for i in 0..buffer.nb_channels as usize {
					buffer.mean[i] =
						buffer.mean[i] + (levels[i] - buffer.mean[i]) / buffer.count as f32;
				}
			} else {
				for i in 0..buffer.nb_channels as usize {
					buffer.mean[i] =
						buffer.mean[i] + (levels[i] - buffer.mean[i]) / stat_mem as f32;
				}
			}
			/*
			for (i, &l) in levels.iter().enumerate() {
				println! {"c: {}, l: {}", i, l};
			}
			*/

			let new_level: Vec<_> = (0..buffer.nb_channels as usize)
				.map(|i| Level {
					val: levels[i],
					mean: buffer.mean[i],
				})
				.collect();
			/*
			for (i, g) in gain.into_iter().enumerate() {
				if i == 0 {
					println! {"c: {}, g: {}", i, g};
				}
			}
			*/

			let mut level = level.lock().unwrap();
			*level = new_level;
			return;
		}
	}
}
