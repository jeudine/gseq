use crate::audio;
use crate::instance::Instance;
use crate::model::{InstanceModel, Model};
use crate::pipeline::Pipeline;
use crate::pipeline::{PipelineError, PipelineGroup};
use rand::prelude::*;
use std::iter::zip;

const COLOR_0: [f32; 4] = [0.1294, 0.2, 0.3882, 0.7];
const COLOR_1: [f32; 4] = [0.0902, 0.3490, 0.2902, 0.7];
const COLOR_2: [f32; 4] = [0.5569, 0.6745, 0.3137, 0.7];
const COLOR_3: [f32; 4] = [0.8275, 0.8157, 0.3098, 0.7];
const COLORS: [[f32; 4]; 4] = [COLOR_0, COLOR_1, COLOR_2, COLOR_3];

fn get_color(rng: &mut ThreadRng) -> [f32; 4] {
	COLORS.choose(rng).unwrap().clone()
}

pub const POST_PATH: &str = "shader/vs_0/post.wgsl";
const NB_DISKS: usize = 4;
const DISK_SPEED: f32 = 0.3;

pub struct State {
	disk_activated: [bool; NB_DISKS],
	disk_start_time: [f32; NB_DISKS],
	disk_duration: [f32; NB_DISKS],
	disk_scale: [f32; NB_DISKS],
	rng: ThreadRng,
}

impl State {
	pub fn new() -> State {
		State {
			disk_activated: [false; NB_DISKS],
			disk_start_time: [0.0; NB_DISKS],
			disk_duration: [0.0; NB_DISKS],
			disk_scale: [0.0; NB_DISKS],
			rng: rand::thread_rng(),
		}
	}

	pub fn update(
		&mut self,
		disk_pipeline: &mut Pipeline,
		time: f32,
		old_audio: audio::Data,
		new_audio: audio::Data,
	) {
		let disks_i = &mut disk_pipeline.instance_models[0].instances;

		let audio_iter = zip(old_audio.gain, new_audio.gain);
		for (o, n) in audio_iter {
			if n > 2.0 && o < 2.0 {
				self.activate_disk(time, disks_i)
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
				instances[i].color = get_color(&mut self.rng);
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
	let disk = Model::new_disk(&device, 200);

	let instances = (0..NB_DISKS).map(|_| Instance::new()).collect();

	let instance_model = InstanceModel::new(disk, instances, &device);

	pipeline_group.add_pipeline(
		vec![instance_model],
		&std::path::PathBuf::from("shader/vs_0/2d_transparent.wgsl"),
		&device,
		&config,
	)?;

	let quad = Model::new_quad(&device);
	let instance = Instance::new();
	let instance_model = InstanceModel::new(quad, vec![instance], &device);

	pipeline_group.add_pipeline(
		vec![instance_model],
		&std::path::PathBuf::from("shader/vs_0/wallpaper_noise_0.wgsl"),
		&device,
		&config,
	)?;

	Ok(())
}
