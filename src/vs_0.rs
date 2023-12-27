use crate::audio;
use crate::instance::Instance;
use crate::model::{InstanceModel, Model};
use crate::pipeline::Pipeline;
use crate::pipeline::{PipelineError, PipelineGroup};
use std::iter::zip;

const COLOR_0: [f32; 4] = [0.1294, 0.2, 0.3882, 1.0];
const COLOR_1: [f32; 4] = [0.0902, 0.3490, 0.2902, 1.0];
const COLOR_2: [f32; 4] = [0.5569, 0.6745, 0.3137, 1.0];
const COLOR_3: [f32; 4] = [0.8275, 0.8157, 0.3098, 1.0];

pub const POST_PATH: &str = "shader/vs_0/post.wgsl";
const NB_DISKS: usize = 4;
const DISK_SPEED: f32 = 2.0;

pub struct State {
	disk_activated: [bool; NB_DISKS],
	disk_start_time: [f32; NB_DISKS],
	disk_duration: [f32; NB_DISKS],
	disk_scale: [f32; NB_DISKS],
}

impl State {
	pub fn new() -> State {
		State {
			disk_activated: [false; NB_DISKS],
			disk_start_time: [0.0; NB_DISKS],
			disk_duration: [0.0; NB_DISKS],
			disk_scale: [0.0; NB_DISKS],
		}
	}

	pub fn update(
		&mut self,
		disk_pipeline: &mut Pipeline,
		time: f32,
		old_audio: audio::Data,
		new_audio: audio::Data,
	) {
		let audio_iter = zip(old_audio.gain, new_audio.gain);
		for (o, n) in audio_iter {
			if n > 2.0 && o < 2.0 {
				self.activate_disk()
			}
		}

		let disks_i = &mut disk_pipeline.instance_models[0].instances;

		for i in 0..NB_DISKS {
			if self.disk_activated[i] {
				let t = time - self.disk_start_time[i];
				if t > self.disk_duration[i] {
					self.disk_activated[i] = false;
					disks_i[i].scale = 0.0;
					continue;
				}
				disks_i[i].scale = self.disk_scale[i] + DISK_SPEED * t;
			}
		}
	}

	fn activate_disk(&mut self) {
		for i in 0..NB_DISKS {
			if !self.disk_activated[i] {
				self.disk_activated[i] = true;
				//TODO
				self.disk_scale[i] = 0.2;
				self.disk_duration[i] = 2.0;
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
	let disc = Model::new_disc(&device, 200);

	let mut i_0 = Instance::new();
	i_0.scale(0.35);
	i_0.translate((0.4, -0.1, 0.0).into());
	i_0.set_color(COLOR_1);
	let mut i_1 = Instance::new();
	i_1.scale(0.4);
	i_1.translate((-0.6, 0.3, 0.0).into());
	i_1.set_color(COLOR_0);

	let mut i_2 = Instance::new();
	i_2.scale(0.3);
	i_2.translate((0.7, 0.6, 0.0).into());
	i_2.set_color(COLOR_2);

	let mut i_3 = Instance::new();
	i_3.scale(0.24);
	i_3.translate((-0.1, -0.6, 0.0).into());
	i_3.set_color(COLOR_3);

	let instance_model = InstanceModel::new(disc, vec![i_0, i_1, i_2, i_3], &device);

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
