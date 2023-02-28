use cpal::{
	traits::{DeviceTrait, HostTrait},
	BufferSize,
};
use std::error::Error;

pub struct FFT {
	chunck_size: u32,
	sample_rate: u32,
	nb_channels: u32,
	freq_limits: Vec<u32>,
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

		Ok(Self {
			chunck_size,
			sample_rate: config.sample_rate().0,
			nb_channels,
			freq_limits: Self::calculate_channel_frequency(min_freq, max_freq, nb_channels),
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
