use crate::instance::Instance;
use crate::instance::Material;
use crate::model::Mesh;
use crate::Model;
use wgpu::util::DeviceExt;

pub struct Group {
	pub model: Vec<(Mesh, Material, wgpu::Buffer)>,
	pub instance: Instance,
}

impl Group {
	pub fn new(file_name: &str, instance: Instance, device: &wgpu::Device) -> Self {
		let model = Model::new(file_name, device).unwrap();
		let model = model
			.meshes
			.into_iter()
			.map(|(mesh, material)| {
				let instance_data = [instance.to_raw(&material)];
				let instance_buffer =
					device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
						label: Some("Instance Buffer"),
						contents: bytemuck::cast_slice(&instance_data),
						usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
					});
				(mesh, material, instance_buffer)
			})
			.collect();
		Self { model, instance }
	}

	pub fn rotate(&mut self, rotation: &cgmath::Basis3<f32>, queue: &mut wgpu::Queue) {
		self.instance.rotate(rotation);
		self.write_buffer(queue);
	}

	pub fn write_buffer(&mut self, queue: &mut wgpu::Queue) {
		for (_mesh, material, buffer) in &mut self.model {
			let instance_data = vec![self.instance.to_raw(material)];
			queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
		}
	}

	pub fn temp_rotate(&mut self, rotation: &cgmath::Basis3<f32>, queue: &mut wgpu::Queue) {
		for (_mesh, material, buffer) in &mut self.model {
			let instance_data = vec![self.instance.to_raw_rotate(material, rotation)];
			queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
		}
	}

	pub fn set_color(&mut self, color: (Option<f32>, Option<f32>, Option<f32>)) {
		for (_mesh, material, _buffer) in &mut self.model {
			if let Some(c) = color.0 {
				material.diffuse.x = c;
				material.spec.x = c;
			}
			if let Some(c) = color.1 {
				material.diffuse.y = c;
				material.spec.y = c;
			}
			if let Some(c) = color.2 {
				material.diffuse.z = c;
				material.spec.z = c;
			}
		}
	}

	pub fn rotate_scale(
		&mut self,
		rotation: &cgmath::Basis3<f32>,
		scale: f32,
		queue: &mut wgpu::Queue,
	) {
		self.instance.rotate(rotation);
		for (_mesh, material, buffer) in &mut self.model {
			let instance_data = vec![self.instance.to_raw_scale(material, scale)];
			queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&instance_data));
		}
	}
}
