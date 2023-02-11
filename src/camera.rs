#[derive(Copy, Clone)]
pub struct Camera {
	pub eye: cgmath::Point3<f32>,
	pub target: cgmath::Point3<f32>,
	pub up: cgmath::Vector3<f32>,
	pub aspect: f32,
	pub fovy: f32,
	pub znear: f32,
	pub zfar: f32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
	view_pos: [f32; 4],
	view_proj: [[f32; 4]; 4],
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

impl From<Camera> for CameraUniform {
	fn from(camera: Camera) -> Self {
		Self {
			view_pos: camera.eye.to_homogeneous().into(),
			view_proj: (OPENGL_TO_WGPU_MATRIX
				* cgmath::perspective(
					cgmath::Deg(camera.fovy),
					camera.aspect,
					camera.znear,
					camera.zfar,
				) * cgmath::Matrix4::look_at_rh(camera.eye, camera.target, camera.up))
			.into(),
		}
	}
}
