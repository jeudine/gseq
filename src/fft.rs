use cpal::{
	platform::Stream,
	traits::{DeviceTrait, HostTrait, StreamTrait},
	FromSample, Sample,
};
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct FFT {
	//sample_rate: u32,
	nb_channels: u32,
	freq_limits: Vec<u32>,
	stream: Stream,
	buffer: Arc<Mutex<Buffer>>,
	pub to_change: Arc<Mutex<bool>>,
}

struct Buffer {
	a: Vec<i16>,
	pos: usize,
	len: usize,
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

		let buffer = Buffer {
			a: vec![0; chunck_size as usize],
			len: chunck_size as usize,
			pos: 0,
		};

		let buffer_arc = Arc::new(Mutex::new(buffer));
		let buffer_arc_2 = buffer_arc.clone();

		let err_fn = move |err| {
			eprintln!("an error occurred on stream: {}", err);
		};

		let stream = match config.sample_format() {
			cpal::SampleFormat::I8 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i8>(data, &buffer_arc),
				err_fn,
				None,
			)?,
			cpal::SampleFormat::I16 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i16>(data, &buffer_arc),
				err_fn,
				None,
			)?,
			cpal::SampleFormat::I32 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<i32>(data, &buffer_arc),
				err_fn,
				None,
			)?,
			cpal::SampleFormat::F32 => device.build_input_stream(
				&config.into(),
				move |data, _: &_| write_input_data::<f32>(data, &buffer_arc),
				err_fn,
				None,
			)?,
			_ => return Err(Box::from("Unsupported sample format")),
		};

		stream.play()?;

		let to_change = false;
		let to_change_arc = Arc::new(Mutex::new(to_change));

		Ok(Self {
			nb_channels,
			freq_limits: Self::calculate_channel_frequency(min_freq, max_freq, nb_channels),
			buffer: buffer_arc_2,
			stream,
			to_change: to_change_arc,
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

type BufferHandle = Arc<Mutex<Buffer>>;

fn write_input_data<T>(input: &[T], buffer: &BufferHandle)
where
	T: Sample,
	i16: FromSample<T>,
{
	if let Ok(mut buffer) = buffer.try_lock() {
		for &sample in input.iter() {
			let pos = buffer.pos;
			buffer.a[pos] = i16::from_sample(sample);
			buffer.pos = pos + 1;
			if buffer.pos == buffer.len {
				buffer.pos = 0;
			}
		}
	}
}
