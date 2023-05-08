use crate::action::Action;
use crate::instance::Instance;
use crate::model::Mesh;
use crate::Model;
use wgpu::util::DeviceExt;

pub struct Group {
	pub model: Vec<(Mesh, wgpu::Buffer)>,
	pub params: Vec<(Instance, Action)>,
}

impl Group {
	pub fn new(file_name: &str, params: &Vec<(Instance, Action)>, device: &wgpu::Device) -> Self {
		let model = Model::new(file_name, device).unwrap();
		let params = params.clone();
		let model = model
			.meshes
			.iter()
			.map(|mesh| {
				let instance_data = params.iter().map(|x| x.0.to_raw()).collect::<Vec<_>>();
				let instance_buffer =
					device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
						label: Some("Instance Buffer"),
						contents: bytemuck::cast_slice(&instance_data),
						usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
					});
				(mesh, instance_buffer)
			})
			.collect();
		Self {
			model,
			params: params.to_vec(),
		}
	}
}
