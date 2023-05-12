use cgmath::{One, Zero};

#[derive(Copy, Clone)]
pub struct Instance {
	pub scale: f32,
	pub rotation: cgmath::Basis3<f32>,
	pub position: cgmath::Vector3<f32>,
}

#[derive(Copy, Clone)]
pub struct Material {
	pub ambient: cgmath::Vector3<f32>,
	pub diffuse: cgmath::Vector3<f32>,
	pub spec: cgmath::Vector3<f32>,
	pub shin: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[allow(dead_code)]
pub struct InstanceRaw {
	ambient: [f32; 3],
	diffuse: [f32; 3],
	spec: [f32; 3],
	shin: f32,
	model: [[f32; 4]; 4],
	normal: [[f32; 3]; 3],
}

impl Instance {
	pub fn new() -> Self {
		let position = cgmath::Vector3::zero();
		let rotation = cgmath::Basis3::one();
		let ambient = cgmath::Vector3::new(0.1, 0.0, 0.0);
		let diffuse = cgmath::Vector3::new(0.5, 0.0, 0.0);
		let spec = cgmath::Vector3::new(1.0, 1.0, 1.0);
		let shin = 16.0;

		Instance {
			position,
			rotation,
			scale: 1.0,
		}
	}

	pub fn to_raw(&self, m: &Material) -> InstanceRaw {
		InstanceRaw {
			ambient: m.ambient.into(),
			diffuse: m.diffuse.into(),
			spec: m.spec.into(),
			shin: m.shin,
			model: (cgmath::Matrix4::from_translation(self.position)
				* cgmath::Matrix4::from(cgmath::Matrix3::from(self.rotation))
				* cgmath::Matrix4::from_scale(self.scale))
			.into(),
			normal: cgmath::Matrix3::from(self.rotation).into(),
		}
	}

	pub fn to_raw_translation(&self, m: &Material, t: cgmath::Vector3<f32>) -> InstanceRaw {
		InstanceRaw {
			ambient: m.ambient.into(),
			diffuse: m.diffuse.into(),
			spec: m.spec.into(),
			shin: m.shin,
			model: (cgmath::Matrix4::from_translation(self.position + t)
				* cgmath::Matrix4::from(cgmath::Matrix3::from(self.rotation))
				* cgmath::Matrix4::from_scale(self.scale))
			.into(),
			normal: cgmath::Matrix3::from(self.rotation).into(),
		}
	}

	pub fn to_raw_rotate(&self, m: &Material, rotation: &cgmath::Basis3<f32>) -> InstanceRaw {
		let rotation = cgmath::Matrix3::from(self.rotation * rotation);
		InstanceRaw {
			ambient: m.ambient.into(),
			diffuse: m.diffuse.into(),
			spec: m.spec.into(),
			shin: m.shin,
			model: (cgmath::Matrix4::from_translation(self.position)
				* cgmath::Matrix4::from(rotation)
				* cgmath::Matrix4::from_scale(self.scale))
			.into(),
			normal: cgmath::Matrix3::from(rotation).into(),
		}
	}
	pub fn to_raw_scale_rotate(
		&self,
		m: &Material,
		scale: f32,
		rotation: &cgmath::Basis3<f32>,
	) -> InstanceRaw {
		let rotation = cgmath::Matrix3::from(self.rotation * rotation);
		InstanceRaw {
			ambient: m.ambient.into(),
			diffuse: m.diffuse.into(),
			spec: m.spec.into(),
			shin: m.shin,
			model: (cgmath::Matrix4::from_translation(self.position)
				* cgmath::Matrix4::from(rotation)
				* cgmath::Matrix4::from_scale(self.scale * scale))
			.into(),
			normal: cgmath::Matrix3::from(rotation).into(),
		}
	}

	pub fn raw_zero() -> InstanceRaw {
		InstanceRaw {
			ambient: [0.0, 0.0, 0.0],
			diffuse: [0.0, 0.0, 0.0],
			spec: [0.0, 0.0, 0.0],
			shin: 0.0,
			model: cgmath::Matrix4::zero().into(),
			normal: cgmath::Matrix3::zero().into(),
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
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
					shader_location: 3,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
					shader_location: 4,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
					shader_location: 5,
					format: wgpu::VertexFormat::Float32,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 10]>() as wgpu::BufferAddress,
					shader_location: 6,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 14]>() as wgpu::BufferAddress,
					shader_location: 7,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 18]>() as wgpu::BufferAddress,
					shader_location: 8,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 22]>() as wgpu::BufferAddress,
					shader_location: 9,
					format: wgpu::VertexFormat::Float32x4,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 26]>() as wgpu::BufferAddress,
					shader_location: 10,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 29]>() as wgpu::BufferAddress,
					shader_location: 11,
					format: wgpu::VertexFormat::Float32x3,
				},
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 32]>() as wgpu::BufferAddress,
					shader_location: 12,
					format: wgpu::VertexFormat::Float32x3,
				},
			],
		}
	}
}
