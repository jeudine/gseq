use crate::audio;
use crate::instance::Instance;
use crate::model::{InstanceModel, Model};
use crate::pipeline::Pipeline;
use crate::pipeline::{PipelineError, PipelineGroup};
use rand::prelude::*;
use std::iter::zip;

/*
const COLOR_0_0: [f32; 4] = [0.1294, 0.5725, 1.0, 0.5];
const COLOR_0_1: [f32; 4] = [0.2196, 0.8980, 0.3020, 0.5];
const COLOR_0_2: [f32; 4] = [0.6118, 1.0, 0.1804, 0.5];
const COLOR_0_3: [f32; 4] = [0.9922, 1.0, 0.0, 0.5];
const COLORS_0: [[f32; 4]; 4] = [COLOR_0_0, COLOR_0_1, COLOR_0_2, COLOR_0_3];
*/

const COLOR_1_0: [f32; 4] = [0.1294, 0.5725, 1.0, 1.0];
const COLOR_1_1: [f32; 4] = [0.2196, 0.8980, 0.3020, 1.0];
const COLOR_1_2: [f32; 4] = [0.6118, 1.0, 0.1804, 1.0];
const COLOR_1_3: [f32; 4] = [0.9922, 1.0, 0.0, 1.0];
const COLORS_1: [[f32; 4]; 4] = [COLOR_1_0, COLOR_1_1, COLOR_1_2, COLOR_1_3];

/*
fn get_color_0(rng: &mut ThreadRng) -> [f32; 4] {
	COLORS_0.choose(rng).unwrap().clone()
}
*/

fn get_color_1(rng: &mut ThreadRng) -> [f32; 4] {
	COLORS_1.choose(rng).unwrap().clone()
}

pub const POST_PATH: &str = "shader/vs_0/post.wgsl";
const NB_DISKS: usize = 4;
const DISK_SPEED: f32 = 0.3;

pub struct State {
	full_activated: (bool, usize),
	full_start_time: f32,
	full_duration: f32,

	disk_activated: [bool; NB_DISKS],
	disk_start_time: [f32; NB_DISKS],
	disk_duration: [f32; NB_DISKS],
	disk_scale: [f32; NB_DISKS],
	rng: ThreadRng,
}

impl State {
	pub fn new() -> State {
		State {
			full_activated: (false, 0),
			full_start_time: 0.0,
			full_duration: 0.0,

			disk_activated: [false; NB_DISKS],
			disk_start_time: [0.0; NB_DISKS],
			disk_duration: [0.0; NB_DISKS],
			disk_scale: [0.0; NB_DISKS],
			rng: rand::thread_rng(),
		}
	}

	pub fn update_2d(
		&mut self,
		pipelines: &mut Vec<Pipeline>,
		time: f32,
		old_audio: &audio::Data,
		new_audio: &audio::Data,
	) {
		self.update_full(&mut pipelines[2], time, old_audio, new_audio);
		self.update_disk(&mut pipelines[3], time, old_audio, new_audio);
	}

	fn update_full(
		&mut self,
		pipeline: &mut Pipeline,
		time: f32,
		old_audio: &audio::Data,
		new_audio: &audio::Data,
	) {
		let audio_iter = zip(old_audio.gain, new_audio.gain);
		for (o, n) in audio_iter {
			if n > 2.0 && o < 2.0 {
				self.activate_full(time, &mut pipeline.instance_models);
				break;
			}
		}

		if self.full_activated.0 {
			let t = time - self.full_start_time;
			if t > self.full_duration {
				self.full_activated.0 = false;
			}
		} else {
			for i_m in &mut pipeline.instance_models {
				i_m.instances[0].scale = 0.0;
			}
		}
	}

	fn activate_full(&mut self, time: f32, i_ms: &mut Vec<InstanceModel>) {
		let i = (0..i_ms.len()).choose(&mut self.rng).unwrap();
		self.full_activated = (true, i);
		self.full_start_time = time;
		self.full_duration = self.rng.gen::<f32>();

		for i_m in &mut *i_ms {
			i_m.instances[0].scale = 0.0;
		}

		let instance = &mut i_ms[i].instances[0];

		instance.color = get_color_1(&mut self.rng);
		instance.scale = self.rng.gen::<f32>() * 0.1 + 0.2;
		instance.position = (
			0.5 - 1.0 * self.rng.gen::<f32>(),
			0.5 - 1.0 * self.rng.gen::<f32>(),
			0.0,
		)
			.into();
	}

	fn update_disk(
		&mut self,
		pipeline: &mut Pipeline,
		time: f32,
		old_audio: &audio::Data,
		new_audio: &audio::Data,
	) {
		let disks_i = &mut pipeline.instance_models[0].instances;

		let audio_iter = zip(old_audio.gain, new_audio.gain);
		for (o, n) in audio_iter {
			if n > 2.0 && o < 2.0 {
				self.activate_disk(time, disks_i);
			}
		}

		for i in 0..NB_DISKS {
			if self.disk_activated[i] {
				let t = time - self.disk_start_time[i];
				if t > self.disk_duration[i] {
					self.disk_activated[i] = false;
					disks_i[i].scale = 0.0;
					continue;
				}
				disks_i[i].scale = self.disk_scale[i] + DISK_SPEED * t;
			} else {
				disks_i[i].scale = 0.0;
			}
		}
	}

	fn activate_disk(&mut self, time: f32, instances: &mut Vec<Instance>) {
		for i in 0..NB_DISKS {
			if !self.disk_activated[i] {
				self.disk_activated[i] = true;
				instances[i].color = get_color_1(&mut self.rng);
				instances[i].position = (
					1.0 - 2.0 * self.rng.gen::<f32>(),
					1.0 - 2.0 * self.rng.gen::<f32>(),
					0.0,
				)
					.into();
				self.disk_start_time[i] = time;
				self.disk_scale[i] = 0.1;
				self.disk_duration[i] = self.rng.gen::<f32>();
				return;
			}
		}
	}
}

pub fn init_2d(
	pipeline_group: &mut PipelineGroup,
	device: &wgpu::Device,
	config: &wgpu::SurfaceConfiguration,
) -> Result<(), PipelineError> {
	let quad = Model::new_quad(&device);
	let instance = Instance::new();
	let instance_model = InstanceModel::new(quad, vec![instance], &device);

	pipeline_group.add_pipeline(
		vec![instance_model],
		&std::path::PathBuf::from("shader/vs_0/wallpaper_noise_0.wgsl"),
		&device,
		&config,
	)?;

	let quad: Model = Model::new_quad(&device);
	let mut instance = Instance::new();
	instance.scale(0.2);
	let instance_model = InstanceModel::new(quad, vec![instance], &device);

	pipeline_group.add_pipeline(
		vec![instance_model],
		&std::path::PathBuf::from("shader/vs_0/2d_logo.wgsl"),
		&device,
		&config,
	)?;

	let quad = Model::new_quad(&device);
	let q_instance = Instance::new();
	let q_instance_model = InstanceModel::new(quad, vec![q_instance], &device);

	let disk = Model::new_disk(&device, 200);
	let d_instance = Instance::new();
	let d_instance_model = InstanceModel::new(disk, vec![d_instance], &device);

	pipeline_group.add_pipeline(
		vec![q_instance_model, d_instance_model],
		&std::path::PathBuf::from("shader/vs_0/2d_full.wgsl"),
		&device,
		&config,
	)?;

	let disk = Model::new_disk(&device, 200);

	let instances = (0..NB_DISKS).map(|_| Instance::new()).collect();

	let instance_model = InstanceModel::new(disk, instances, &device);

	pipeline_group.add_pipeline(
		vec![instance_model],
		&std::path::PathBuf::from("shader/vs_0/2d_transparent.wgsl"),
		&device,
		&config,
	)?;
	Ok(())
}

pub fn init_3d(
	pipeline_group: &mut PipelineGroup,
	device: &wgpu::Device,
	config: &wgpu::SurfaceConfiguration,
) -> Result<(), PipelineError> {
	// let disc = Model::new_disk(&device, 200);/
	let icosphere = Model::import("models/mfroom_3d.obj", device)?;
	let instance = Instance::new();
	let instance_model = InstanceModel::new(icosphere, vec![instance], &device);
	pipeline_group.add_pipeline(
		vec![instance_model],
		&std::path::PathBuf::from("shader/vs_0/3d.wgsl"),
		&device,
		&config,
	)?;

	Ok(())
}
