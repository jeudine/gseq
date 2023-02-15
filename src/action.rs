use cgmath;

#[derive(Copy, Clone)]
pub enum Action {
	Still,
	Rotate(cgmath::Vector3<f32>, cgmath::Rad<f32>),
}
