use crate::instance::Instance;
use crate::Model;
use wgpu::util::DeviceExt;

pub struct Group {
	pub model: Model,
	pub instances: Vec<Instance>,
	pub instance_buffer: wgpu::Buffer,
}

impl Group {
	pub fn new(file_name: &str, nb_instances: u32, device: &wgpu::Device) -> Self {
		let model = Model::new(file_name, device).unwrap();
		let instances = vec![Instance::new(); nb_instances as usize];
		let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
		let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Instance Buffer"),
			contents: bytemuck::cast_slice(&instance_data),
			usage: wgpu::BufferUsages::VERTEX,
		});
		Self {
			model,
			instances,
			instance_buffer,
		}
	}
}
