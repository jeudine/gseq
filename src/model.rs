use std::error::Error;
use std::mem;
use tobj::load_obj;
use wgpu::util::DeviceExt;

//TODO Normals

pub struct Model {
	pub meshes: Vec<Mesh>,
}

pub struct Mesh {
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
	pub num_elements: u32,
}

impl Model {
	pub fn new(file_name: &str, device: &wgpu::Device) -> Result<Model, Box<dyn Error>> {
		let (models, _) = load_obj(
			file_name,
			&tobj::LoadOptions {
				triangulate: true,
				single_index: true,
				..Default::default()
			},
		)?;

		//TODO: default case
		//let material = material.unwrap();

		let meshes = models
			.into_iter()
			.map(|m| {
				let vertices = if m.mesh.normals.len() == m.mesh.positions.len() {
					(0..m.mesh.positions.len() / 3)
						.map(|i| {
							[
								m.mesh.positions[i * 3],
								m.mesh.positions[i * 3 + 1],
								m.mesh.positions[i * 3 + 2],
								m.mesh.normals[i * 3],
								m.mesh.normals[i * 3 + 1],
								m.mesh.normals[i * 3 + 2],
							]
						})
						.collect::<Vec<[f32; 6]>>()
				} else {
					(0..m.mesh.positions.len() / 3)
						.map(|i| {
							[
								m.mesh.positions[i * 3],
								m.mesh.positions[i * 3 + 1],
								m.mesh.positions[i * 3 + 2],
								m.mesh.positions[i * 3],
								m.mesh.positions[i * 3 + 1],
								m.mesh.positions[i * 3 + 2],
							]
						})
						.collect::<Vec<[f32; 6]>>()
				};

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

		Ok(Self { meshes })
	}

	pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		wgpu::VertexBufferLayout {
			// 6 x f32
			array_stride: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[
				// Position
				wgpu::VertexAttribute {
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x3,
				},
				// Normal
				wgpu::VertexAttribute {
					offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
					shader_location: 1,
					format: wgpu::VertexFormat::Float32x3,
				},
			],
		}
	}
}
