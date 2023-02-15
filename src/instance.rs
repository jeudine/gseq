use cgmath::{One, Zero};

#[derive(Copy, Clone)]
pub struct Instance {
	pub scale: f32,
	pub rotation: cgmath::Basis3<f32>,
	pub position: cgmath::Vector3<f32>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[allow(dead_code)]
pub struct InstanceRaw {
	model: [[f32; 4]; 4],
	normal: [[f32; 3]; 3],
}

impl Instance {
	pub fn new() -> Self {
		let position = cgmath::Vector3::zero();
		let rotation = cgmath::Basis3::one();
		Instance {
			position,
			rotation,
			scale: 1.0,
		}
	}

	pub fn to_raw(&self) -> InstanceRaw {
		InstanceRaw {
			model: (cgmath::Matrix4::from_translation(self.position)
				* cgmath::Matrix4::from(cgmath::Matrix3::from(self.rotation))
				* cgmath::Matrix4::from_scale(self.scale))
			.into(),
			normal: cgmath::Matrix3::from(self.rotation).into(),
		}
	}

	pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		use std::mem;
		wgpu::VertexBufferLayout {
			array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Instance,
			attributes: &[
				wgpu::VertexAttribute {
					offset: 0,
					shader_location: 2,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
					shader_location: 3,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
					shader_location: 4,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
					shader_location: 5,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
					shader_location: 6,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
					shader_location: 7,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
					shader_location: 8,
					format: wgpu::VertexFormat::Float32x3,
				},
			],
		}
	}
}
