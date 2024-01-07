use crate::audio;
use crate::camera::{Camera, CameraUniform};
use crate::instance::Instance;
use crate::pipeline;
use crate::texture::{Texture, TextureError};
use std::iter;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use thiserror::Error;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::vs_0;

#[derive(Error, Debug)]
pub enum DisplayError {
	#[error("Failed to create a pipeline")]
	PipelineCreation(#[from] pipeline::PipelineError),
	#[error("Failed to request an adapter")]
	AdapterRequest,
	#[error("Failed to request a device")]
	DeviceRequest(#[from] wgpu::RequestDeviceError),
	#[error("Failed to load a texture")]
	TextureLoad(#[from] TextureError),
}

pub struct Display {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	pub size: winit::dpi::PhysicalSize<u32>,
	window: Window,

	start_time: Instant,

	// Pipelines
	pipeline_groups: Vec<pipeline::PipelineGroup>,
	pipeline_post: pipeline::PipelinePost,

	// Camera
	camera: Camera,

	// Textures
	depth_texture: Texture,
	framebuffer: Texture,

	// Buffers
	audio_buffer: wgpu::Buffer,
	time_buffer: wgpu::Buffer,
	size_buffer: wgpu::Buffer,
	camera_buffer: wgpu::Buffer,

	// Bind groups
	bind_groups: Vec<wgpu::BindGroup>,
	texture_bind_group_layout: wgpu::BindGroupLayout,

	// Audio
	audio_data: audio::Data,

	// State
	vs_0_state: vs_0::State,
}

impl Display {
	pub async fn new(window: Window) -> Result<Self, DisplayError> {
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
			.ok_or(DisplayError::AdapterRequest)?;

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
			.await?;

		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface.get_supported_formats(&adapter)[0],
			width: size.width,
			height: size.height,
			present_mode: wgpu::PresentMode::Fifo,
			alpha_mode: wgpu::CompositeAlphaMode::Auto,
		};
		surface.configure(&device, &config);

		// Camera bind group
		let camera = Camera {
			// +z is out of the screen
			eye: (0.0, 0.0, 5.0).into(),
			// have it look at the origin
			target: (0.0, 0.0, 0.0).into(),
			// which way is "up"
			up: cgmath::Vector3::unit_y(),
			aspect: config.width as f32 / config.height as f32,
			fovy: 50.0,
			znear: 0.1,
			zfar: 100.0,
		};

		let camera_uniform: CameraUniform = camera.into();
		let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("view_proj_buffer"),
			contents: bytemuck::cast_slice(&[camera_uniform]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let camera_bind_group_layout =
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
				label: Some("camera_bind_group_layout"),
			});

		let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &camera_bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: camera_buffer.as_entire_binding(),
			}],
			label: Some("camera_bind_group"),
		});

		// Audio bindings
		let audio_data = audio::Data::new();
		let audio_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("audio_buffer"),
			contents: bytemuck::cast_slice(&[audio_data]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		// Time bindings
		let start_time = Instant::now();
		let time = start_time.elapsed().as_secs_f32();

		let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("time_buffer"),
			contents: bytemuck::cast_slice(&[time]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		// Size bindings
		let size_data: [u32; 2] = [size.width, size.height];
		let size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("size_buffer"),
			contents: bytemuck::cast_slice(&[size_data]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		// Universal bind group
		let universal_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 1,
						visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 2,
						visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
				],
				label: Some("universal_bind_group_layout"),
			});

		let universal_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &universal_bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: audio_buffer.as_entire_binding(),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: time_buffer.as_entire_binding(),
				},
				wgpu::BindGroupEntry {
					binding: 2,
					resource: size_buffer.as_entire_binding(),
				},
			],
			label: Some("universal_bind_group"),
		});

		// Textures bind group
		let framebuffer = Texture::new_framebuffer(
			&device,
			(config.width, config.height),
			"framebuffer texture",
		);

		let logo = Texture::new_image("text/mf_room_logo.png", &device, &queue, "logo")?;

		let texture_bind_group_layout: wgpu::BindGroupLayout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[
					Texture::create_texture_bind_group_layout_entry(0),
					Texture::create_sampler_bind_group_layout_entry(1),
				],
				label: Some("texture_bind_group_layout"),
			});

		let texture_bind_group_0 =
			framebuffer.create_bind_group(&device, &texture_bind_group_layout);

		let texture_bind_group_1 = logo.create_bind_group(&device, &texture_bind_group_layout);

		let depth_texture = Texture::new_depth(&device, &config, "depth_texture");

		let bind_groups = vec![
			universal_bind_group,
			camera_bind_group,
			texture_bind_group_0,
			texture_bind_group_1,
		];
		let bind_group_layouts = vec![
			&universal_bind_group_layout,
			&camera_bind_group_layout,
			&texture_bind_group_layout,
			&texture_bind_group_layout,
		];

		// Create the 2d pipeline group
		let bind_group_indices_2d = vec![0, 3];
		let mut pipeline_group_2d =
			pipeline::PipelineGroup::new_2d(&bind_group_layouts, bind_group_indices_2d, &device);
		vs_0::init_2d(&mut pipeline_group_2d, &device, &config)?;

		// Create the 3d piepline group
		let bind_group_indices_3d = vec![0, 1];
		let mut pipeline_group_3d =
			pipeline::PipelineGroup::new_3d(&bind_group_layouts, bind_group_indices_3d, &device);
		vs_0::init_3d(&mut pipeline_group_3d, &device, &config)?;

		let pipeline_groups = vec![pipeline_group_2d, pipeline_group_3d];

		// Create postpipeline
		let bind_group_indices_post = vec![0, 2];

		let pipeline_post = pipeline::PipelinePost::new(
			&bind_group_layouts,
			bind_group_indices_post,
			&device,
			&config,
			&std::path::PathBuf::from(vs_0::POST_PATH),
		)?;

		let vs_0_state = vs_0::State::new();

		Ok(Self {
			surface,
			device,
			queue,
			config,
			size,
			window,
			start_time,
			pipeline_groups,
			pipeline_post,
			camera,
			depth_texture,
			framebuffer,
			audio_buffer,
			time_buffer,
			size_buffer,
			camera_buffer,
			bind_groups,
			texture_bind_group_layout,
			audio_data,
			vs_0_state,
		})
	}

	pub fn window(&self) -> &Window {
		&self.window
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;

			// Create new textures with new size
			self.depth_texture = Texture::new_depth(&self.device, &self.config, "depth_texture");
			self.framebuffer = Texture::new_framebuffer(
				&self.device,
				(new_size.width, new_size.height),
				"framebuffer texture",
			);
			// Update the bind group of relevant textures
			self.bind_groups[2] = self
				.framebuffer
				.create_bind_group(&self.device, &self.texture_bind_group_layout);
			self.surface.configure(&self.device, &self.config);

			// Update window size
			let size_data: [u32; 2] = [self.size.width, self.size.height];
			self.queue
				.write_buffer(&self.size_buffer, 0, bytemuck::cast_slice(&[size_data]));

			// Update camera
			self.camera.aspect = self.config.width as f32 / self.config.height as f32;
			let camera_uniform: CameraUniform = self.camera.into();
			self.queue.write_buffer(
				&self.camera_buffer,
				0,
				bytemuck::cast_slice(&[camera_uniform]),
			);
		}
	}

	pub fn update(&mut self, audio: &Arc<Mutex<audio::Data>>) {
		// println!("{:?}", self.frame_buffer);
		// Update audio
		let audio_data = *audio.lock().unwrap();
		self.queue
			.write_buffer(&self.audio_buffer, 0, bytemuck::cast_slice(&[audio_data]));

		// Update time
		let time = self.start_time.elapsed().as_secs_f32();
		self.queue
			.write_buffer(&self.time_buffer, 0, bytemuck::cast_slice(&[time]));

		// Update the InstanceModels
		self.vs_0_state.update_2d(
			&mut self.pipeline_groups[0].pipelines,
			time,
			&self.audio_data,
			&audio_data,
		);

		for p_g in &self.pipeline_groups {
			for p in &p_g.pipelines {
				for i_m in &p.instance_models {
					let instance_data = i_m
						.instances
						.iter()
						.map(Instance::to_raw)
						.collect::<Vec<_>>();
					self.queue.write_buffer(
						&i_m.instance_buffer,
						0,
						bytemuck::cast_slice(&instance_data),
					);
				}
			}
		}

		self.audio_data = audio_data;
	}

	pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
					view: &self.framebuffer.view(),
					resolve_target: None,
					ops: wgpu::Operations {
						//load: wgpu::LoadOp::Load,
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.0,
							g: 0.0,
							b: 0.0,
							a: 1.0,
						}),
						store: true,
					},
				})],
				depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
					view: &self.depth_texture.view(),
					depth_ops: Some(wgpu::Operations {
						load: wgpu::LoadOp::Clear(1.0),
						store: true,
					}),
					stencil_ops: None,
				}),
			});

			for g in &self.pipeline_groups {
				let bg_indices = g.layout.get_bind_group_indices();
				for (u, i) in bg_indices.iter().enumerate() {
					render_pass.set_bind_group(u as u32, &self.bind_groups[*i], &[]);
				}
				for p in &g.pipelines {
					p.draw(&mut render_pass);
				}
			}
		}

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Post Processing"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						//load: wgpu::LoadOp::Load,
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.0,
							g: 0.0,
							b: 0.0,
							a: 1.0,
						}),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});

			let bg_indices = self.pipeline_post.get_bind_group_indices();
			for (u, i) in bg_indices.iter().enumerate() {
				render_pass.set_bind_group(u as u32, &self.bind_groups[*i], &[]);
			}
			self.pipeline_post.draw(&mut render_pass);
		}

		self.queue.submit(iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}
