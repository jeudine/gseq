use crate::action::Action;
use crate::camera::{Camera, CameraUniform};
use crate::fft;
use crate::group::Group;
use crate::instance::Instance;
use crate::item::Item;
use crate::light::Light;
use crate::model::Model;
use crate::texture::Texture;
use cgmath::{Basis3, Deg, Euler, Rotation3};
use rand::Rng;
use std::iter;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use wgpu::util::DeviceExt;
use winit::window::Window;

pub struct Display {
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	config: wgpu::SurfaceConfiguration,
	pub size: winit::dpi::PhysicalSize<u32>,
	render_pipeline: wgpu::RenderPipeline,
	depth_texture: Texture,
	pub groups: Vec<Group>,
	pub lights: Light,
	#[allow(dead_code)]
	camera: Camera,
	#[allow(dead_code)]
	view_proj_buffer: wgpu::Buffer,
	bind_group: wgpu::BindGroup,
	window: Window,
	start_time: Instant,
	cur_fft_instance: u32,
	cur_pupil_pos: cgmath::Vector3<f32>,
	cur_pupil_rot: cgmath::Euler<cgmath::Deg<f32>>,
	above_05: bool,
	was_in_drop: bool,
}

impl Display {
	pub async fn new(window: Window, items: Vec<Item>) -> Self {
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
			eye: (0.0, 0.0, 5.0).into(),
			// have it look at the origin
			target: (0.0, 0.0, 0.0).into(),
			// which way is "up"
			up: cgmath::Vector3::unit_y(),
			aspect: config.width as f32 / config.height as f32,
			fovy: 45.0,
			znear: 0.1,
			zfar: 100.0,
		};

		let camera_uniform: CameraUniform = camera.into();

		let view_proj_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("view_proj_buffer"),
			contents: bytemuck::cast_slice(&[camera_uniform]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let vp_bind_group_layout =
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
				label: Some("mv_bind_group_layout"),
			});

		let light_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[
					wgpu::BindGroupLayoutEntry {
						binding: 0,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 1,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 2,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 3,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 4,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 5,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
					wgpu::BindGroupLayoutEntry {
						binding: 6,
						visibility: wgpu::ShaderStages::FRAGMENT,
						ty: wgpu::BindingType::Buffer {
							ty: wgpu::BufferBindingType::Uniform,
							has_dynamic_offset: false,
							min_binding_size: None,
						},
						count: None,
					},
				],
				label: Some("light_bind_group_layout"),
			});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &vp_bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: view_proj_buffer.as_entire_binding(),
			}],
			label: Some("vp_bind_group"),
		});

		let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

		let render_pipeline_layout =
			device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some("Render Pipeline Layout"),
				// TODO: if we want multiple lights add more light bind groups
				bind_group_layouts: &[&vp_bind_group_layout, &light_bind_group_layout],
				push_constant_ranges: &[],
			});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[Model::desc(), Instance::desc()],
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

		let lights = Light::new(&device, &light_bind_group_layout);
		let groups: Vec<Group> = items
			.iter()
			.map(|x| Group::new(&x.file_name, &x.params, &device))
			.collect();

		/*
		let mut nb_fft_instances = 0;
		for g in &groups {
			for (_, a) in &g.params {
				if let Action::FFT = a {
					nb_fft_instances += 1;
				}
			}
		}
		*/
		Self {
			surface,
			device,
			queue,
			config,
			size,
			render_pipeline,
			depth_texture,
			groups,
			lights: lights,
			camera,
			view_proj_buffer,
			bind_group,
			window,
			start_time: Instant::now(),
			//nb_fft_instances,
			cur_fft_instance: 0,
			cur_pupil_pos: cgmath::Vector3::new(0.0, 0.0, 0.0),
			cur_pupil_rot: cgmath::Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)),
			above_05: false,
			was_in_drop: true,
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
			self.depth_texture =
				Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
			self.surface.configure(&self.device, &self.config);
		}
	}

	pub fn update(&mut self, phase: &Arc<Mutex<fft::Phase>>) {
		let time = self.start_time.elapsed().as_secs_f32();

		for g in &mut self.groups {
			//TODO: only one group
			let (instance, action) = g.params[0];
			match action {
				Action::Still => {
					for (_mesh, material, buffer) in &mut g.model {
						let instance_data = vec![instance.to_raw(material)];
						self.queue
							.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
					}
				}
				Action::Rotate(v, s) => {
					let a = s * time;
					for (_mesh, material, buffer) in &mut g.model {
						let rotation = cgmath::Basis3::from_axis_angle(v, a);
						let instance_data = vec![instance.to_raw_rotate(material, &rotation)];
						self.queue
							.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
					}
				}
				Action::FFT => {}
			}
		}

		let pupil = &self.groups[0];
		let (pupil_instance, _) = pupil.params[0];
		use cgmath::Vector3;
		let pupil_pos = vec![
			Vector3::new(0.0, 0.0, 0.0),
			Vector3::new(-0.4, 0.0, 0.0),
			Vector3::new(0.4, 0.0, 0.0),
			Vector3::new(0.0, 0.4, 0.0),
			Vector3::new(0.0, -0.4, 0.0),
		];
		let pupil_rot = vec![
			Euler::new(Deg(0.0), Deg(0.0), Deg(0.0)),
			Euler::new(Deg(30.0), Deg(0.0), Deg(0.0)),
			Euler::new(Deg(-30.0), Deg(0.0), Deg(0.0)),
			Euler::new(Deg(0.0), Deg(30.0), Deg(0.0)),
			Euler::new(Deg(0.0), Deg(-30.0), Deg(0.0)),
		];

		let pupil_ring = &self.groups[1];
		let (pupil_ring_instance, _) = pupil_ring.params[0];

		let outside = &self.groups[2];
		let (outside_instance, _) = outside.params[0];

		let iris2 = &self.groups[3];
		let (iris2_instance, _) = iris2.params[0];

		let iris3 = &self.groups[4];
		let (iris3_instance, _) = iris3.params[0];

		//FFT elements
		let phase = phase.lock().unwrap();

		match phase.state {
			fft::State::Break(b) => match b {
				fft::Break::State0 | fft::Break::State1 => {
					for (_mesh, material, buffer) in &pupil.model {
						let instance_data = vec![pupil_instance.to_raw(material)];
						self.queue
							.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
					}

					for (_mesh, material, buffer) in &pupil_ring.model {
						let instance_data = vec![pupil_ring_instance.to_raw(material)];
						self.queue
							.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
					}

					for (_mesh, material, buffer) in &outside.model {
						if self.was_in_drop {
							let instance_data = vec![outside_instance.to_raw_rotate(
								material,
								&cgmath::Basis3::from(Euler {
									x: Deg(110.0),
									y: Deg(0.0),
									z: Deg(0.0),
								}),
							)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
							self.was_in_drop = false;
						} else if phase.gains[0] > 3.5 {
							let instance_data = vec![outside_instance.to_raw_rotate(
								material,
								&cgmath::Basis3::from(Euler {
									x: Deg(0.0),
									y: Deg(0.0),
									z: Deg(0.0),
								}),
							)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}
					}
					for (_mesh, material, buffer) in &iris2.model {
						let color = activation_func(phase.gains[3], -0.5, 0.5, 0.0, 1.0);
						let mut new_material = material.clone();
						new_material.diffuse.x = color;
						new_material.spec.y = color;
						let instance_data = vec![iris2_instance.to_raw(&new_material)];
						self.queue
							.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
					}

					for (_mesh, material, buffer) in &iris3.model {
						let color = activation_func(phase.gains[2], -0.5, 0.5, 0.0, 1.0);
						let mut new_material = material.clone();
						new_material.diffuse.z = color;
						new_material.spec.x = color;
						let instance_data = vec![iris3_instance
							.to_raw_translate(&new_material, cgmath::Vector3::new(0.0, 0.0, -0.3))];
						self.queue
							.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
					}
				}

				fft::Break::State2 | fft::Break::State3 => {}
			},
			fft::State::Drop(d) => {
				self.was_in_drop = true;
				match d {
					fft::Drop::State0 => {
						for (_mesh, material, buffer) in &pupil.model {
							let instance_data = vec![pupil_instance.to_raw(material)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, material, buffer) in &pupil_ring.model {
							let instance_data = vec![pupil_ring_instance.to_raw(material)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, material, buffer) in &outside.model {
							let x = activation_func(phase.gains[0], -0.5, 0.5, 0.0, 110.0);
							let instance_data = vec![outside_instance.to_raw_rotate(
								material,
								&cgmath::Basis3::from(Euler {
									x: Deg(x),
									y: Deg(0.0),
									z: Deg(0.0),
								}),
							)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}
						for (_mesh, material, buffer) in &iris2.model {
							let color = activation_func(phase.gains[3], -0.5, 0.5, 0.0, 1.0);
							let mut new_material = material.clone();
							new_material.diffuse.y = color;
							new_material.spec.y = color;
							let instance_data = vec![iris2_instance.to_raw(&new_material)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, material, buffer) in &iris3.model {
							let color = activation_func(phase.gains[2], -0.5, 0.5, 0.0, 1.0);
							let mut new_material = material.clone();
							new_material.diffuse.x = color;
							new_material.spec.x = color;
							let instance_data = vec![iris3_instance.to_raw_translate(
								&new_material,
								cgmath::Vector3::new(0.0, 0.0, -0.3),
							)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}
					}
					fft::Drop::State1 => {
						if phase.gains[0] > 0.5 && !self.above_05 {
							let mut rng = rand::thread_rng();
							let i = rng.gen_range(0..5);
							self.cur_pupil_pos = pupil_pos[i];
						}
						for (_mesh, material, buffer) in &pupil.model {
							let instance_data =
								vec![pupil_instance.to_raw_translate(material, self.cur_pupil_pos)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, _material, buffer) in &pupil_ring.model {
							let instance_data = vec![Instance::raw_zero()];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, material, buffer) in &outside.model {
							let instance_data = vec![outside_instance.to_raw_rotate(
								material,
								&cgmath::Basis3::from(cgmath::Euler {
									x: cgmath::Deg(20.0),
									y: cgmath::Deg(0.0),
									z: cgmath::Deg(0.0),
								}),
							)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, material, buffer) in &iris2.model {
							let color = activation_func(phase.gains[3], -0.5, 0.5, 0.0, 1.0);
							let mut new_material = material.clone();
							new_material.diffuse.x = color;
							new_material.spec.x = color;
							let instance_data = vec![iris2_instance.to_raw(&new_material)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, material, buffer) in &iris3.model {
							let color = activation_func(phase.gains[2], -0.5, 0.5, 0.0, 1.0);
							let mut new_material = material.clone();
							new_material.diffuse.z = color;
							new_material.spec.z = color;
							let instance_data = vec![iris3_instance.to_raw_translate(
								&new_material,
								cgmath::Vector3::new(0.0, 0.0, -0.3),
							)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}
					}
					fft::Drop::State2 => {
						if phase.gains[0] > 0.5 && !self.above_05 {
							let mut rng = rand::thread_rng();
							let i = rng.gen_range(0..5);
							self.cur_pupil_rot = pupil_rot[i];
						}

						for (_mesh, material, buffer) in &pupil.model {
							let instance_data = vec![pupil_instance.to_raw(material)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, _material, buffer) in &pupil_ring.model {
							let instance_data = vec![Instance::raw_zero()];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, material, buffer) in &outside.model {
							let instance_data = vec![outside_instance.to_raw_rotate(
								material,
								&cgmath::Basis3::from(cgmath::Euler {
									x: cgmath::Deg(20.0),
									y: cgmath::Deg(0.0),
									z: cgmath::Deg(0.0),
								}),
							)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, material, buffer) in &iris2.model {
							let color = activation_func(phase.gains[3], -0.5, 0.5, 0.0, 1.0);
							let mut new_material = material.clone();
							new_material.diffuse.x = color;
							new_material.spec.x = color;
							let instance_data = vec![iris2_instance.to_raw(&new_material)];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}

						for (_mesh, material, buffer) in &iris3.model {
							let color = activation_func(phase.gains[2], -0.5, 0.5, 0.0, 1.0);
							let mut new_material = material.clone();
							new_material.diffuse.z = color;
							new_material.spec.z = color;
							let instance_data = vec![iris3_instance
								.to_raw_rotate(&new_material, &Basis3::from(self.cur_pupil_rot))];
							self.queue.write_buffer(
								&buffer,
								0,
								bytemuck::cast_slice(&instance_data),
							);
						}
					}

					_ => {}
				}
			}
		}
		self.above_05 = if phase.gains[0] > 0.5 { true } else { false };

		/*
		let eye_lid = &mut self.groups[0];
		let (eye_lid_instance, _) = &eye_lid.params[0];

		for (mesh, material, buffer) in &mut eye_lid.model {
			let x = activation_func(phase.gains[0], -0.5, 0.5, 46.0, -50.0);
			let instance_data = vec![eye_lid_instance.to_raw_rotate(
				material,
				&cgmath::Basis3::from(Euler {
					x: Deg(x),
					y: Deg(0.0),
					z: Deg(0.0),
				}),
			)];
			self.queue
				.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
		}
		*/

		/*
			*/

		/*

		let rot_speed = cgmath::Rad(0.8);
		let rot_vector = cgmath::Vector3::new(0.0, 1.0, 0.0);
		let mut count_fft_instance = 0;
		let time = self.start_time.elapsed().as_secs_f32();
		for g in &mut self.groups {
			for (_, m, b) in &mut g.model {
				let instance_data = g
					.params
					.iter()
					.map(|p| match p.1 {
						Action::Still => p.0.to_raw(m),
						Action::Rotate(v, s) => {
							let a = s * time;
							let rotation = cgmath::Basis3::from_axis_angle(v, a);
							p.0.to_raw_rotate(&m, &rotation)
						}
						Action::FFT => {
							let i = if count_fft_instance == self.cur_fft_instance {
								let a = rot_speed * time;
								let rotation = cgmath::Basis3::from_axis_angle(rot_vector, a);
								p.0.to_raw_scale_rotate(&m, (phase.gains[0] / 3.0).exp(), &rotation)
							} else {
								Instance::raw_zero()
							};
							count_fft_instance += 1;
							i
						}
					})
					.collect::<Vec<_>>();
				self.queue
					.write_buffer(&b, 0, bytemuck::cast_slice(&instance_data));
			}
		}
		*/
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
				depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
					view: &self.depth_texture.view,
					depth_ops: Some(wgpu::Operations {
						load: wgpu::LoadOp::Clear(1.0),
						store: true,
					}),
					stencil_ops: None,
				}),
			});

			render_pass.set_pipeline(&self.render_pipeline);

			for g in &self.groups {
				for (me, _, b) in &g.model {
					render_pass.set_vertex_buffer(1, b.slice(..));
					render_pass.set_vertex_buffer(0, me.vertex_buffer.slice(..));
					render_pass
						.set_index_buffer(me.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
					render_pass.set_bind_group(0, &self.bind_group, &[]);
					render_pass.set_bind_group(1, &self.lights.bind_group, &[]);
					render_pass.draw_indexed(0..me.num_elements, 0, 0..g.params.len() as _);
				}
			}
		}

		self.queue.submit(iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}

fn activation_func(x: f32, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> f32 {
	if x < min_x {
		min_y
	} else if x > max_x {
		max_y
	} else {
		let a = (max_y - min_y) / (max_x - min_x);
		let b = min_y - a * min_x;
		a * x + b
	}
}
