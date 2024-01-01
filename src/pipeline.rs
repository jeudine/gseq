use fs_err as fs;
use thiserror::Error;

use crate::instance::InstanceRaw;
use crate::model::InstanceModel;
use crate::model::Model;
use crate::texture::Texture;

#[derive(Error, Debug)]
pub enum PipelineError {
	#[error("Failed to read shader")]
	Reading(#[from] std::io::Error),
}

pub struct Pipeline {
	render_pipeline: wgpu::RenderPipeline,
	pub instance_models: Vec<InstanceModel>,
}

pub struct PipelineGroup {
	pub layout: Layout,
	pub pipelines: Vec<Pipeline>,
}

pub struct PipelinePost {
	pub layout: LayoutInner,
	pub render_pipeline: wgpu::RenderPipeline,
	model: Model, //quad
}

pub enum Layout {
	Pipeline2D(LayoutInner),
	Pipeline3D(LayoutInner),
}

pub struct LayoutInner {
	pipeline_layout: wgpu::PipelineLayout,
	pub bind_group_indices: Vec<usize>,
}

impl PipelineGroup {
	pub fn new_2d(
		bind_group_layouts: &Vec<&wgpu::BindGroupLayout>,
		bind_group_indices: Vec<usize>,
		device: &wgpu::Device,
	) -> Self {
		let layout = Layout::Pipeline2D(LayoutInner::new(
			bind_group_layouts,
			bind_group_indices,
			device,
		));
		PipelineGroup {
			layout,
			pipelines: vec![],
		}
	}

	pub fn new_3d(
		bind_group_layouts: &Vec<&wgpu::BindGroupLayout>,
		bind_group_indices: Vec<usize>,
		device: &wgpu::Device,
	) -> Self {
		let layout = Layout::Pipeline3D(LayoutInner::new(
			bind_group_layouts,
			bind_group_indices,
			device,
		));
		PipelineGroup {
			layout,
			pipelines: vec![],
		}
	}

	pub fn add_pipeline(
		&mut self,
		instance_models: Vec<InstanceModel>,
		shader_path: &std::path::Path,
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
	) -> Result<(), PipelineError> {
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Shader"),
			source: wgpu::ShaderSource::Wgsl(fs::read_to_string(shader_path)?.into()),
		});

		let pipeline_layout = self.layout.get_pipeline_layout();
		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("2D Render Pipeline"),
			layout: Some(pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[Model::desc(), InstanceRaw::desc()],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: config.format,
					blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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

		let pipeline = Pipeline {
			render_pipeline,
			instance_models,
		};

		self.pipelines.push(pipeline);

		Ok(())
	}
}

impl LayoutInner {
	fn new(
		bind_group_layout: &Vec<&wgpu::BindGroupLayout>,
		bind_group_indices: Vec<usize>,
		device: &wgpu::Device,
	) -> Self {
		let bind_groups: Vec<_> = bind_group_indices
			.iter()
			.map(|i| bind_group_layout[*i])
			.collect();

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: bind_groups.as_slice(),
			push_constant_ranges: &[],
		});

		Self {
			pipeline_layout,
			bind_group_indices: bind_group_indices,
		}
	}
}

impl Layout {
	pub fn get_bind_group_indices(&self) -> &Vec<usize> {
		match self {
			Layout::Pipeline2D(l) | Layout::Pipeline3D(l) => &l.bind_group_indices,
		}
	}

	fn get_pipeline_layout(&self) -> &wgpu::PipelineLayout {
		match self {
			Layout::Pipeline2D(l) | Layout::Pipeline3D(l) => &l.pipeline_layout,
		}
	}
}

impl Pipeline {
	/*
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
			label: Some("3D Render Pipeline"),
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
	*/

	pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
		render_pass.set_pipeline(&self.render_pipeline);
		for instance_model in &self.instance_models {
			for mesh in &instance_model.model.meshes {
				render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
				render_pass.set_vertex_buffer(1, instance_model.instance_buffer.slice(..));
				let nb_instances = instance_model.instances.len();
				render_pass
					.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
				render_pass.draw_indexed(0..mesh.num_elements, 0, 0..nb_instances as _);
			}
		}
	}
}

impl PipelinePost {
	pub fn new(
		bind_group_layout: &Vec<&wgpu::BindGroupLayout>,
		bind_group_indices: Vec<usize>,
		device: &wgpu::Device,
		config: &wgpu::SurfaceConfiguration,
		shader_path: &std::path::Path,
	) -> Result<Self, PipelineError> {
		let bind_groups: Vec<_> = bind_group_indices
			.iter()
			.map(|i| bind_group_layout[*i])
			.collect();

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("Render Pipeline Layout"),
			bind_group_layouts: bind_groups.as_slice(),
			push_constant_ranges: &[],
		});

		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Shader"),
			source: wgpu::ShaderSource::Wgsl(fs::read_to_string(shader_path)?.into()),
		});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Post Processing Render Pipeline"),
			layout: Some(&pipeline_layout),
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
					blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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
			depth_stencil: None,
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			multiview: None,
		});

		let layout = LayoutInner {
			pipeline_layout: pipeline_layout,
			bind_group_indices: bind_group_indices,
		};

		Ok(PipelinePost {
			layout,
			render_pipeline,
			model: Model::new_quad(device),
		})
	}

	pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
		render_pass.set_pipeline(&self.render_pipeline);
		for mesh in &self.model.meshes {
			render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
			render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
			render_pass.draw_indexed(0..mesh.num_elements, 0, 0..1 as _);
		}
	}

	pub fn get_bind_group_indices(&self) -> &Vec<usize> {
		&self.layout.bind_group_indices
	}
}
