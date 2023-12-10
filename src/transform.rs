const MAX_NB_SHAPES: usize = 8;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Transforms_2D {
	// Pos_x, Pos_y, Size, Rot
	pos_size: [[f32; 4]; MAX_NB_SHAPES],
}

pub struct Transform_2D {
	pub pos_x: f32,
	pub pos_y: f32,
	pub size: f32,
	pub rot: f32,
}

impl Transforms_2D {
	pub fn new() -> Self {
		Self {
			pos_size: [[0.0; 4]; MAX_NB_SHAPES],
		}
	}

	pub const fn size() -> usize {
		MAX_NB_SHAPES
	}

	pub fn get(&self, i: usize) -> Transform_2D {
		let t = self.pos_size[i];
		Transform_2D {
			pos_x: t[0],
			pos_y: t[1],
			size: t[2],
			rot: t[3],
		}
	}

	pub fn set(&mut self, i: usize, transform: Transform_2D) {
		self.pos_size[i] = [
			transform.pos_x,
			transform.pos_y,
			transform.size,
			transform.rot,
		];
	}
}
