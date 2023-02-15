use cgmath;

#[derive(Copy, Clone)]
pub enum Action {
	Still,
	Rotate(cgmath::Quaternion<f32>),
}
