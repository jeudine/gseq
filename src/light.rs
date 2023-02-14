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
	pub light_buffer: wgpu::Buffer,
	pub bind_group: BindGroup,
}

impl Light {
	pub fn new(device: &wgpu::Device, bind_group_layout: &BindGroupLayout) -> Light {
		let light_uniform = LightUniform {
			position: [0.0, 0.0, 10.0],
			_padding: 0,
			color: [1.0, 1.0, 1.0],
			_padding2: 0,
		};
		let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Light VB"),
			contents: bytemuck::cast_slice(&[light_uniform]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: light_buffer.as_entire_binding(),
			}],
			label: None,
		});
		Light {
			light_buffer,
			bind_group,
		}
	}
}
