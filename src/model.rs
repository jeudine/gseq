use crate::Matrix;
use std::error::Error;
use tobj::load_obj;
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout};

//TODO Normals

pub struct Model {
	pub meshes: Vec<Mesh>,
	pub transform: Matrix,
	pub transform_buffer: wgpu::Buffer,
	pub bind_group: BindGroup,
}

pub struct Mesh {
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
	pub num_elements: u32,
}

impl Model {
	pub fn new(
		file_name: &str,
		device: &wgpu::Device,
		bind_group_layout: &BindGroupLayout,
	) -> Result<Model, Box<dyn Error>> {
		let (models, _) = load_obj(
			file_name,
			&tobj::LoadOptions {
				triangulate: true,
				single_index: true,
				..Default::default()
			},
		)?;
		let meshes = models
			.into_iter()
			.map(|m| {
				let vertices = (0..m.mesh.positions.len() / 3)
					.map(|i| {
						[
							m.mesh.positions[i * 3],
							m.mesh.positions[i * 3 + 1],
							m.mesh.positions[i * 3 + 2],
						]
					})
					.collect::<Vec<[f32; 3]>>();
				let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: Some(&format!("{:?} Vertex Buffer", file_name)),
					contents: bytemuck::cast_slice(&vertices),
					usage: wgpu::BufferUsages::VERTEX,
				});
				let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
					label: Some(&format!("{:?} Index Buffer", file_name)),
					contents: bytemuck::cast_slice(&m.mesh.indices),
					usage: wgpu::BufferUsages::INDEX,
				});
				Mesh {
					vertex_buffer,
					index_buffer,
					num_elements: m.mesh.indices.len() as u32,
				}
			})
			.collect::<Vec<_>>();

		let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("transform_buffer"),
			contents: bytemuck::cast_slice(&[Matrix::identity()]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: transform_buffer.as_entire_binding(),
			}],
			label: Some("mvp_bind_group"),
		});

		Ok(Model {
			meshes,
			transform: Matrix::identity(),
			transform_buffer,
			bind_group,
		})
	}

	pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		wgpu::VertexBufferLayout {
			array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[wgpu::VertexAttribute {
				offset: 0,
				shader_location: 0,
				format: wgpu::VertexFormat::Float32x3,
			}],
		}
	}
}
