use crate::model::Model;
use crate::texture::Texture;
use std::fs;

struct Pipeline {
	render_pipeline: wgpu::RenderPipeline,
	models: Vec<Model>,
}

struct Layouts {
	layout_3d: wgpu::PipelineLayout,
	layout_2d: wgpu::PipelineLayout,
}

impl Pipeline {
	fn init_layouts(
		camera_bind_group_layout: &wgpu::BindGroupLayout,
		audio_bind_group_layout: &wgpu::BindGroupLayout,
	) -> Layouts {
	}
	fn new_3d(
		pipeline_layouts: PipelineLayouts,
		device: wgpu::Device,
		models: Vec<Model>,
		shader_path: &std::path::Path,
		camera_bind_group: &wgpu::BindGroup,
		audio_bind_group: &wgpu::BindGroup,
		time_bind_group: &wgpu::BindGroup,
		depth_texture: &Texture,
	) {
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Shader"),
			//TODO: unwrap
			source: wgpu::ShaderSource::Wgsl(fs::read_to_string(shader_path).unwrap().into()),
		});

		let render_pipeline_layout =
			device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some("Render Pipeline Layout"),
				bind_group_layouts: &[
					&camera_bind_group_layout,
					&audio_bind_group_layout,
					&time_bind_group_layout,
				],
				push_constant_ranges: &[],
			});
	}
	fn new_2d() {}
}
