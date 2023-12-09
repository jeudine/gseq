use crate::model::Model;
use crate::texture::Texture;
use std::fs;

struct Pipeline {
	render_pipeline: wgpu::RenderPipeline,
	models: Vec<Model>,
}

impl Pipeline {
	fn new_3d(
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
	}
	fn new_2d() {}
}
