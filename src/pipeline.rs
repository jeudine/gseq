use fs_err as fs;
use thiserror::Error;

use crate::model::Model;
use crate::texture::Texture;

#[derive(Error, Debug)]
pub enum PipelineError {
	#[error("Failed to read shader")]
	Reading(#[from] std::io::Error),
}

pub struct Pipeline {
	render_pipeline: wgpu::RenderPipeline,
	models: Vec<Model>,
}

pub struct Layouts {
	layout_2d: wgpu::PipelineLayout,
	layout_3d: wgpu::PipelineLayout,
}

impl Layouts {
	pub fn new(
		camera_bind_group_layout: &wgpu::BindGroupLayout,
		audio_bind_group_layout: &wgpu::BindGroupLayout,
		time_bind_group_layout: &wgpu::BindGroupLayout,
		device: &wgpu::Device,
	) -> Layouts {
		let layout_3d = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[
				&camera_bind_group_layout,
				&audio_bind_group_layout,
				&time_bind_group_layout,
			],
			push_constant_ranges: &[],
		});
		let layout_2d = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: &[&audio_bind_group_layout, &time_bind_group_layout],
			push_constant_ranges: &[],
		});
		Layouts {
			layout_2d,
			layout_3d,
		}
	}
}

impl Pipeline {
	pub fn new_3d(
		layouts: &Layouts,
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		models: Vec<Model>,
		shader_path: &std::path::Path,
	) -> Result<Pipeline, PipelineError> {
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Shader"),
			source: wgpu::ShaderSource::Wgsl(fs::read_to_string(shader_path)?.into()),
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&layouts.layout_3d),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[Model::desc()],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: config.format,
					blend: Some(wgpu::BlendState {
						color: wgpu::BlendComponent::REPLACE,
						alpha: wgpu::BlendComponent::REPLACE,
					}),
					write_mask: wgpu::ColorWrites::ALL,
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: Some(wgpu::Face::Back),
				// Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
				// or Features::POLYGON_MODE_POINT
				polygon_mode: wgpu::PolygonMode::Fill,
				// Requires Features::DEPTH_CLIP_CONTROL
				unclipped_depth: false,
				// Requires Features::CONSERVATIVE_RASTERIZATION
				conservative: false,
			},
			depth_stencil: Some(wgpu::DepthStencilState {
				format: Texture::DEPTH_FORMAT,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::Less, // 1.
				stencil: wgpu::StencilState::default(),     // 2.
				bias: wgpu::DepthBiasState::default(),
			}),
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			multiview: None,
		});

		Ok(Pipeline {
			render_pipeline,
			models,
		})
	}

	fn new_2d() {}

	pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
		render_pass.set_pipeline(&self.render_pipeline);
		for model in &self.models {
			for mesh in &model.meshes {
				render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
				render_pass
					.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
				render_pass.draw_indexed(0..mesh.num_elements, 0, 0..1 as _);
			}
		}
	}
}
