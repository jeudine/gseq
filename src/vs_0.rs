use crate::audio;
use crate::instance::Instance;
use crate::model::{InstanceModel, Model};
use crate::pipeline::{PipelineError, PipelineGroup};
use std::iter::{zip, Zip};

const COLOR_0: [f32; 4] = [0.1294, 0.2, 0.3882, 1.0];
const COLOR_1: [f32; 4] = [0.0902, 0.3490, 0.2902, 1.0];
const COLOR_2: [f32; 4] = [0.5569, 0.6745, 0.3137, 1.0];
const COLOR_3: [f32; 4] = [0.8275, 0.8157, 0.3098, 1.0];

pub const POST_PATH: &str = "shader/vs_0/post.wgsl";

struct Animation {
	active: bool,
	start_time: f32,
	duration: f32,
	scale_factor: f32,
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

/*
pub fn update_2d(pipeline_group: &mut PipelineGroup, time: f32, audio: Zip<f32, f32>) {
	let instance_models_0 = &pipeline_group.pipelines[0].instance_models;
	for (a0, a1) in audio {
		if a > 1.0 {}
	}
}
*/
