use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout};

// Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct LightUniform {
	position: [f32; 3],
	_padding: u32,
	color: [f32; 3],
	_padding2: u32,
}

pub struct Light {
	pub light_buffer_0: wgpu::Buffer,
	pub light_buffer_1: wgpu::Buffer,
	pub light_buffer_2: wgpu::Buffer,
	pub bind_group: BindGroup,
}

impl Light {
	pub fn new(device: &wgpu::Device, bind_group_layout: &BindGroupLayout) -> Light {
		let light_uniform_0 = LightUniform {
			position: [-10.0, 8.66, 10.0],
			_padding: 0,
			color: [1.0, 0.0, 0.0],
			_padding2: 0,
		};
		let light_uniform_1 = LightUniform {
			position: [10.0, 8.66, 10.0],
			_padding: 0,
			color: [0.0, 1.0, 0.0],
			_padding2: 0,
		};
		let light_uniform_2 = LightUniform {
			position: [0.0, -8.66, 10.0],
			_padding: 0,
			color: [0.0, 0.0, 1.0],
			_padding2: 0,
		};

		let light_buffer_0 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Light VB"),
			contents: bytemuck::cast_slice(&[light_uniform_0]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let light_buffer_1 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Light VB"),
			contents: bytemuck::cast_slice(&[light_uniform_1]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let light_buffer_2 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Light VB"),
			contents: bytemuck::cast_slice(&[light_uniform_2]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: light_buffer_0.as_entire_binding(),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: light_buffer_1.as_entire_binding(),
				},
				wgpu::BindGroupEntry {
					binding: 2,
					resource: light_buffer_2.as_entire_binding(),
				},
			],
			label: None,
		});

		Light {
			light_buffer_0,
			light_buffer_1,
			light_buffer_2,
			bind_group,
		}
	}
}
