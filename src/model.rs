use crate::instance::Instance;
use std::f32::consts::PI;
use thiserror::Error;
use tobj::load_obj;
use wgpu::util::DeviceExt;

#[derive(Error, Debug)]
pub enum ModelError {
	#[error("Failed to read obj file")]
	Reading(#[from] tobj::LoadError),
}

pub struct InstanceModel {
	pub model: Model,
	pub instances: Vec<Instance>,
	pub instance_buffer: wgpu::Buffer,
}

pub struct Model {
	pub meshes: Vec<Mesh>,
}

pub struct Mesh {
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
	pub num_elements: u32,
}

impl InstanceModel {
	pub fn new(model: Model, instances: Vec<Instance>, device: &wgpu::Device) -> Self {
		let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
		let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Instance Buffer"),
			contents: bytemuck::cast_slice(&instance_data),
			usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
		});
		//println!("{:?}", instances);
		//panic! {}
		Self {
			model,
			instances,
			instance_buffer,
		}
	}
}

impl Model {
	pub fn new_quad(device: &wgpu::Device) -> Model {
		let vertices: Vec<[f32; 3]> = vec![
			[-1.0, 1.0, 0.0],
			[1.0, 1.0, 0.0],
			[-1.0, -1.0, 0.0],
			[1.0, -1.0, 0.0],
		];
		let indices: Vec<u32> = vec![0, 2, 1, 1, 2, 3];

		Self::points_to_model(device, &vertices, &indices)
	}

	pub fn new_disk(device: &wgpu::Device, nb_points: u32) -> Model {
		let nb_points_r = if nb_points < 4 { 4 } else { nb_points };
		let mut vertices: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]];

		let nb_points_f = nb_points_r as f32;
		let pi_2 = 2.0 * PI;

		let mut indices: Vec<u32> = vec![];
		for i in 0..nb_points_r {
			let angle = pi_2 * i as f32 / nb_points_f;
			vertices.push([angle.cos(), angle.sin(), 0.0]);
			indices.extend_from_slice(&[0, i + 1, i + 2]);
		}
		indices.extend_from_slice(&[0, nb_points_r, 1]);

		Self::points_to_model(device, &vertices, &indices)
	}

	fn points_to_model(
		device: &wgpu::Device,
		vertices: &Vec<[f32; 3]>,
		indices: &Vec<u32>,
	) -> Self {
		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some(&format!("Quad Vertex Buffer")),
			contents: bytemuck::cast_slice(vertices),
			usage: wgpu::BufferUsages::VERTEX,
		});

		let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some(&format!("Quad Index Buffer")),
			contents: bytemuck::cast_slice(indices),
			usage: wgpu::BufferUsages::INDEX,
		});

		let mesh = Mesh {
			vertex_buffer,
			index_buffer,
			num_elements: indices.len() as u32,
		};

		Model { meshes: vec![mesh] }
	}

	pub fn import(file_name: &str, device: &wgpu::Device) -> Result<Model, ModelError> {
		let (model, _) = load_obj(
			file_name,
			&tobj::LoadOptions {
				triangulate: true,
				single_index: true,
				..Default::default()
			},
		)?;

		let meshes = model
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
				let mesh = Mesh {
					vertex_buffer,
					index_buffer,
					num_elements: m.mesh.indices.len() as u32,
				};

				let id = match m.mesh.material_id {
					Some(x) => x,
					None => 0,
				};

				mesh
			})
			.collect::<Vec<_>>();

		Ok(Self { meshes })
	}

	pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
		wgpu::VertexBufferLayout {
			// 3 x f32
			array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
			step_mode: wgpu::VertexStepMode::Vertex,
			attributes: &[
				// Position
				wgpu::VertexAttribute {
					offset: 0,
					shader_location: 0,
					format: wgpu::VertexFormat::Float32x3,
				},
			],
		}
	}
}
