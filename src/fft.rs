use cpal::{
	platform::Stream,
	traits::{DeviceTrait, HostTrait, StreamTrait},
	FromSample, Sample,
};

use realfft::{num_complex::Complex, RealFftPlanner, RealToComplex};
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct FFT {
	//sample_rate: u32,
	nb_channels: u32,
	freq_limits: Vec<u32>,
	stream: Stream,
	pub levels: Arc<Mutex<bool>>,
}

struct Buffer {
	input: Vec<f32>,
	output: Vec<Complex<f32>>,
	scratch: Vec<Complex<f32>>,
	pos: usize,
	len: usize,
	r2c: Arc<dyn RealToComplex<f32>>,
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
		let mut input = r2c.make_input_vec();
		let mut output = r2c.make_output_vec();
		let mut scratch = r2c.make_scratch_vec();

		let mut buffer = Buffer {
			input,
			output,
			scratch,
			len: chunck_size as usize,
			pos: 0,
			r2c,
		};

		let err_fn = move |err| {
			eprintln!("an error occurred on stream: {}", err);
		};

		let levels = false;
		let levels_arc = Arc::new(Mutex::new(levels));
		let levels_arc2 = levels_arc.clone();

		let stream = match config.sample_format() {
			cpal::SampleFormat::I8 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i8>(data, &mut buffer, &levels_arc2),
				err_fn,
				None,
			)?,
			cpal::SampleFormat::I16 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i16>(data, &mut buffer, &levels_arc2),
				err_fn,
				None,
			)?,
			cpal::SampleFormat::I32 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i32>(data, &mut buffer, &levels_arc2),
				err_fn,
				None,
			)?,
			cpal::SampleFormat::F32 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<f32>(data, &mut buffer, &levels_arc2),
				err_fn,
				None,
			)?,
			_ => return Err(Box::from("Unsupported sample format")),
		};

		stream.play()?;

		Ok(Self {
			nb_channels,
			freq_limits: Self::calculate_channel_frequency(min_freq, max_freq, nb_channels),
			stream,
			levels: levels_arc,
		})
	}

	fn calculate_channel_frequency(min_freq: u32, max_freq: u32, nb_channels: u32) -> Vec<u32> {
		let nb_octaves = log_2(max_freq / min_freq);
		let nb_octaves_per_channel = nb_octaves / nb_channels;
		let freq_limits = (0..nb_channels + 1)
			.map(|i| min_freq * 2_u32.pow(nb_octaves_per_channel * i))
			.collect();
		freq_limits
	}
}

const fn num_bits<T>() -> usize {
	std::mem::size_of::<T>() * 8
}

fn log_2(x: u32) -> u32 {
	num_bits::<u32>() as u32 - x.leading_zeros() - 1
}

fn write_input_data<T>(input: &[T], buffer: &mut Buffer, levels: &Arc<Mutex<bool>>)
where
	T: Sample,
	f32: FromSample<T>,
{
	// every 2 because stereo
	for &sample in input.iter().step_by(2) {
		let pos = buffer.pos;
		buffer.input[pos] = f32::from_sample(sample);
		buffer.pos = pos + 1;
		if buffer.pos == buffer.len {
			buffer.pos = 0;
			let mut real_planner = RealFftPlanner::<f32>::new();
			let r2c = real_planner.plan_fft_forward(buffer.len);
			r2c.process_with_scratch(&mut buffer.input, &mut buffer.output, &mut buffer.scratch);
			/*
			for (i, el) in buffer.output.iter().enumerate() {
				println!("{}: {}", i, el.norm());
			}
			*/
			// update stats
			// compute levels
			break;
		}
	}
}
