pub mod light;
pub mod model;
use crate::light::Light;
use crate::model::Model;
use std::iter;

use wgpu::util::DeviceExt;
use winit::{
	event::*,
	event_loop::{ControlFlow, EventLoop},
	window::{Window, WindowBuilder},
};

struct Camera {
	eye: cgmath::Point3<f32>,
	target: cgmath::Point3<f32>,
	up: cgmath::Vector3<f32>,
	aspect: f32,
	fovy: f32,
	znear: f32,
	zfar: f32,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl Camera {
	fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
		let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
		let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
		return OPENGL_TO_WGPU_MATRIX * proj * view;
	}
}

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Matrix4 {
	// We can't use cgmath with bytemuck directly so we'll have
	// to convert the Matrix4 into a 4x4 f32 array
	m: [[f32; 4]; 4],
}

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Matrix3 {
	// We can't use cgmath with bytemuck directly so we'll have
	// to convert the Matrix4 into a 4x4 f32 array
	m: [[f32; 3]; 3],
}

impl Matrix4 {
	fn identity() -> Self {
		use cgmath::SquareMatrix;
		Self {
			m: cgmath::Matrix4::identity().into(),
		}
	}

	fn zero() -> Self {
		use cgmath::Zero;
		Self {
			m: cgmath::Matrix4::zero().into(),
		}
	}

	fn update_view_proj(&mut self, camera: &Camera) {
		self.m = camera.build_view_projection_matrix().into();
	}
}

impl Matrix3 {
	fn identity() -> Self {
		use cgmath::SquareMatrix;
		Self {
			m: cgmath::Matrix3::identity().into(),
		}
	}

	fn zero() -> Self {
		use cgmath::Zero;
		Self {
			m: cgmath::Matrix3::zero().into(),
		}
	}
}

struct State {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	size: winit::dpi::PhysicalSize<u32>,
	render_pipeline: wgpu::RenderPipeline,
	model: Vec<Model>,
	light: Vec<Light>,
	camera: Camera,
	camera_uniform: Matrix4,
	view_proj_buffer: wgpu::Buffer,
	bind_group: wgpu::BindGroup,
	window: Window,
}

impl State {
	async fn new(window: Window, file_name: &str) -> Self {
		let size = window.inner_size();

		// The instance is a handle to our GPU
		// BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
		let instance = wgpu::Instance::new(wgpu::Backends::all());

		// # Safety
		//
		// The surface needs to live as long as the window that created it.
		// State owns the window so this should be safe.
		let surface = unsafe { instance.create_surface(&window) };

		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::default(),
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			})
			.await
			.unwrap();

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					label: None,
					features: wgpu::Features::empty(),
					// WebGL doesn't support all of wgpu's features, so if
					// we're building for the web we'll have to disable some.
					limits: wgpu::Limits::default(),
				},
				None, // Trace path
			)
			.await
			.unwrap();

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface.get_supported_formats(&adapter)[0],
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
			alpha_mode: wgpu::CompositeAlphaMode::Auto,
		};
		surface.configure(&device, &config);

		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some("Shader"),
			source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
		});

		let camera = Camera {
			// position the camera one unit up and 2 units back
			// +z is out of the screen
			eye: (0.0, 1.0, 10.0).into(),
			// have it look at the origin
			target: (0.0, 0.0, 0.0).into(),
			// which way is "up"
			up: cgmath::Vector3::unit_y(),
			aspect: config.width as f32 / config.height as f32,
			fovy: 45.0,
			znear: 0.1,
			zfar: 100.0,
		};

		let mut camera_uniform = Matrix4::identity();

		camera_uniform.update_view_proj(&camera);

		let view_proj_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("view_proj_buffer"),
			contents: bytemuck::cast_slice(&[camera_uniform]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let mvp_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
				label: Some("mvp_bind_group_layout"),
			});

		let light_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
				label: Some("light_bind_group_layout"),
			});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &mvp_bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: view_proj_buffer.as_entire_binding(),
			}],
			label: Some("mvp_bind_group"),
		});

		let render_pipeline_layout =
			device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some("Render Pipeline Layout"),
				// TODO: if we want multiple lights add more light bind groups
				bind_group_layouts: &[
					&mvp_bind_group_layout,
					&mvp_bind_group_layout,
					&light_bind_group_layout,
				],
				push_constant_ranges: &[],
			});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
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
			depth_stencil: None,
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			// If the pipeline will be used with a multiview render pass, this
			// indicates how many array layers the attachments will have.
			multiview: None,
		});

		let model0 = Model::new(file_name, &device, &mvp_bind_group_layout).unwrap();
		let model1 = Model::new(file_name, &device, &mvp_bind_group_layout).unwrap();

		let light0 = Light::new(&device, &light_bind_group_layout);

		Self {
			surface,
			device,
			queue,
			config,
			size,
			render_pipeline,
			model: vec![model0, model1],
			light: vec![light0],
			camera,
			camera_uniform,
			view_proj_buffer,
			bind_group,
			window,
		}
	}

	pub fn window(&self) -> &Window {
		&self.window
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	#[allow(unused_variables)]
	fn input(&mut self, event: &WindowEvent) -> bool {
		false
	}

	fn update(&mut self) {
		self.queue.write_buffer(
			&self.model[0].transform_buffer,
			0,
			bytemuck::cast_slice(&Matrix4::zero().m),
		);
		/*
		self.queue.write_buffer(
			&self.model[1].transform_buffer,
			0,
			bytemuck::cast_slice(&Matrix::zero().m),
		);
		*/
	}

	fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;
		let view = output
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self
			.device
			.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				label: Some("Render Encoder"),
			});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.1,
							g: 0.2,
							b: 0.3,
							a: 1.0,
						}),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});

			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.set_bind_group(0, &self.bind_group, &[]);
			render_pass.set_bind_group(2, &self.light[0].bind_group, &[]);
			for model in &self.model {
				render_pass.set_bind_group(1, &model.bind_group, &[]);
				for m in &model.meshes {
					render_pass.set_vertex_buffer(0, m.vertex_buffer.slice(..));
					render_pass
						.set_index_buffer(m.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
					render_pass.draw_indexed(0..m.num_elements, 0, 0..1);
				}
			}
		}

		self.queue.submit(iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}

pub async fn run(file_name: &str) {
	let event_loop = EventLoop::new();
	let window = WindowBuilder::new().build(&event_loop).unwrap();

	let mut state = State::new(window, file_name).await;

	event_loop.run(move |event, _, control_flow| {
		match event {
			Event::WindowEvent {
				ref event,
				window_id,
			} if window_id == state.window().id() => {
				if !state.input(event) {
					match event {
						WindowEvent::CloseRequested
						| WindowEvent::KeyboardInput {
							input:
								KeyboardInput {
									state: ElementState::Pressed,
									virtual_keycode: Some(VirtualKeyCode::Escape),
									..
								},
							..
						} => *control_flow = ControlFlow::Exit,
						WindowEvent::Resized(physical_size) => {
							state.resize(*physical_size);
						}
						WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
							// new_inner_size is &mut so w have to dereference it twice
							state.resize(**new_inner_size);
						}
						_ => {}
					}
				}
			}
			Event::RedrawRequested(window_id) if window_id == state.window().id() => {
				state.update();
				match state.render() {
					Ok(_) => {}
					// Reconfigure the surface if it's lost or outdated
					Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
						state.resize(state.size)
					}
					// The system is out of memory, we should probably quit
					Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
					// We're ignoring timeouts
					Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
				}
			}
			Event::MainEventsCleared => {
				// RedrawRequested will only trigger once, unless we manually
				// request it.
				state.window().request_redraw();
			}
			_ => {}
		}
	});
}
