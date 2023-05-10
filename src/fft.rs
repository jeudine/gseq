use core::f32::consts::PI;
use cpal::{
	platform::Stream,
	traits::{DeviceTrait, HostTrait, StreamTrait},
	FromSample, Sample,
};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use realfft::{num_complex::Complex, RealFftPlanner, RealToComplex};
use std::collections::VecDeque;
use std::error::Error;
use std::sync::{Arc, Mutex};
#[cfg(feature = "profile")]
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub enum Drop {
	State0,
	State1,
	State2,
	State3,
}

impl Distribution<Drop> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Drop {
		match rng.gen_range(0..=3) {
			0 => Drop::State0,
			1 => Drop::State1,
			2 => Drop::State2,
			_ => Drop::State3,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Break {
	State0,
	State1,
	State2,
	State3,
}

impl Distribution<Break> for Standard {
	fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Break {
		match rng.gen_range(0..=3) {
			0 => Break::State0,
			1 => Break::State1,
			2 => Break::State2,
			_ => Break::State3,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum State {
	Break(Break),
	Drop(Drop),
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
	global_mean: Vec<f32>,
	global_var: Vec<f32>,
	count: u64,
	nb_channels: u32,
	index_limits: Vec<usize>,
	stat_window: Vec<VecDeque<f32>>,
	stat_window_size: u32,
	state: State,
}

/*
#[derive(Debug, Clone, Copy)]
pub struct Level {
	pub val: f32,
	pub mean: f32,
	pub sd: f32,
}
*/

#[derive(Debug, Clone)]
pub struct Phase {
	pub gains: Vec<f32>,
	pub state: State,
	pub reset: bool,
}

pub fn init(
	chunck_size: u32,
	nb_channels: u32,
	min_freq: u32,
	max_freq: u32,
) -> Result<(Arc<Mutex<Phase>>, Stream), Box<dyn Error>> {
	let host = cpal::default_host();
	let device = host
		.default_input_device()
		.expect("failed to find input device");

	println!("Input device: {}", device.name()?);

	let config = device
		.default_input_config()
		.expect("Failed to get default input config");
	println!("Default input config: {:?}", config);

	let stat_window_size = 50;

	let mut real_planner = RealFftPlanner::<f32>::new();
	let r2c = real_planner.plan_fft_forward(chunck_size as usize);
	let input = r2c.make_input_vec();
	let output = r2c.make_output_vec();
	let scratch = r2c.make_scratch_vec();
	let mean = vec![0.0; nb_channels as usize];
	let var = vec![0.0; nb_channels as usize];
	let stat_window = vec![VecDeque::new(); nb_channels as usize];
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
		var,
		global_mean: vec![0.0; nb_channels as usize],
		global_var: vec![0.0; nb_channels as usize],
		count: 0,
		window: hanning_window,
		nb_channels,
		index_limits: calculate_channel_index(
			min_freq,
			max_freq,
			nb_channels,
			config.sample_rate().0,
			chunck_size,
		),
		stat_window,
		stat_window_size,
		state: State::Break(Break::State0),
	};

	let err_fn = move |err| {
		eprintln!("an error occurred on stream: {}", err);
	};

	let phase = Phase {
		gains: vec![0.0; nb_channels as usize],
		state: State::Break(Break::State0),
		reset: false,
	};

	let phase_arc = Arc::new(Mutex::new(phase));
	let phase_arc2 = phase_arc.clone();

	let stream = match config.sample_format() {
		cpal::SampleFormat::I8 => device.build_input_stream(
			&config.into(),
			move |data, _: &_| handle_input::<i8>(data, &mut buffer, &phase_arc2),
			err_fn,
			None,
		)?,
		cpal::SampleFormat::I16 => device.build_input_stream(
			&config.into(),
			move |data, _: &_| handle_input::<i16>(data, &mut buffer, &phase_arc2),
			err_fn,
			None,
		)?,
		cpal::SampleFormat::I32 => device.build_input_stream(
			&config.into(),
			move |data, _: &_| handle_input::<i32>(data, &mut buffer, &phase_arc2),
			err_fn,
			None,
		)?,
		cpal::SampleFormat::F32 => device.build_input_stream(
			&config.into(),
			move |data, _: &_| handle_input::<f32>(data, &mut buffer, &phase_arc2),
			err_fn,
			None,
		)?,
		_ => return Err(Box::from("Unsupported sample format")),
	};

	stream.play()?;
	Ok((phase_arc, stream))
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

	/*
	for i in &index_limits {
		println!(
			"index: {}, freq: {}",
			i,
			i * sample_rate as usize / chunck_size as usize
		);
	}
	*/

	index_limits
}

fn handle_input<T>(input: &[T], buffer: &mut Buffer, phase: &Arc<Mutex<Phase>>)
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

			// compute levels
			let levels: Vec<_> = (0..buffer.nb_channels as usize)
				.map(|x| {
					(buffer.index_limits[x] + 1..buffer.index_limits[x + 1])
						.fold(0.0, |acc, i| acc + buffer.output[i].norm())
				})
				.collect();

			// update mean, sd and stat_window
			let tmp_inv = 1.0 / (buffer.stat_window_size) as f32;

			if buffer.count == 1 {
				for i in 0..buffer.nb_channels as usize {
					buffer.global_mean[i] = levels[i];
				}
			} else {
				for i in 0..buffer.nb_channels as usize {
					let tmp_0: f32 = buffer.count as f32;
					let tmp_1: f32 = (tmp_0 - 1.0) / tmp_0;
					buffer.global_mean[i] = tmp_1 * buffer.global_mean[i] + levels[i] / tmp_0;
					buffer.global_var[i] = tmp_1 * buffer.global_var[i]
						+ (levels[i] - buffer.global_mean[i]).powi(2) / tmp_0;
				}
			}

			// Initialization
			if buffer.count <= buffer.stat_window_size as u64 {
				for i in 0..buffer.nb_channels as usize {
					buffer.stat_window[i].push_front(levels[i]);
					buffer.mean[i] += tmp_inv * levels[i];
					buffer.var[i] += tmp_inv * levels[i].powi(2);
					if buffer.count == buffer.stat_window_size as u64 {
						buffer.var[i] -= buffer.mean[i].powi(2);
					}
				}
			} else {
				for i in 0..buffer.nb_channels as usize {
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

			/*
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
						sd: buffer.var[i].sqrt(),
					})
					.collect();
				let mut level = level.lock().unwrap();
				*level = new_level;
				return;
			}
			*/

			/*
			for (i, &l) in levels.iter().enumerate() {
				println! {"c: {}, l: {}", i, l};
			}
			*/

			// Update State
			let mean_low = buffer.mean[0];
			let global_mean_low = buffer.global_mean[0];
			let global_sd_low = buffer.global_var[0].sqrt();
			let val = (mean_low - global_mean_low) / global_sd_low;

			buffer.state = if val > 0.2 {
				if let State::Break(_) = buffer.state {
					let state = State::Drop(rand::random());
					println!("{:?}", state);
					state
				} else {
					buffer.state
				}
			} else if val < 0.2 {
				if let State::Drop(_) = buffer.state {
					let state = State::Break(rand::random());
					println!("{:?}", state);
					state
				} else {
					buffer.state
				}
			} else {
				buffer.state
			};

			let gains: Vec<_> = (0..buffer.nb_channels as usize)
				.map(|i| (levels[i] - buffer.mean[i]) / buffer.var[i].sqrt())
				.collect();
			/*
			for (i, g) in gain.into_iter().enumerate() {
				if i == 0 {
					println! {"c: {}, g: {}", i, g};
				}
			}
			*/

			let mut phase = phase.lock().unwrap();
			if phase.reset {
				buffer.global_mean = buffer.mean.clone();
				buffer.global_var = buffer.var.clone();
				buffer.state = State::Break(Break::State0);
				println!("Global man and var reset");
			}
			#[cfg(feature = "profile")]
			profile(&new_level);

			*phase = Phase {
				gains,
				state: buffer.state,
				reset: false,
			};
			return;
		}
	}
}

pub fn reset_global(phase: &Arc<Mutex<Phase>>) {
	let mut phase = phase.lock().unwrap();
	phase.reset = true;
}

#[cfg(feature = "profile")]
fn profile(level: &Vec<Level>) {
	let start = SystemTime::now();
	let since_the_epoch: f64 = start
		.duration_since(UNIX_EPOCH)
		.expect("Time went backwards")
		.as_secs_f64();
	println!(
		"{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
		since_the_epoch,
		level[0].val,
		level[0].mean,
		level[0].sd,
		level[1].val,
		level[1].mean,
		level[1].sd,
		level[2].val,
		level[2].mean,
		level[2].sd,
		level[3].val,
		level[3].mean,
		level[3].sd
	);
}
