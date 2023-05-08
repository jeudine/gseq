use cgmath::Basis3;
use cgmath::One;
use cgmath::{Deg, Euler};
use gseq::run;
use gseq::Action;
use gseq::Instance;
use gseq::Item;

fn main() {
	/*
	let instance0 = Instance {
		position: cgmath::Vector3::new(15.0, 0.0, -30.0),
		rotation: cgmath::Basis3::one(),
		scale: 0.05,
		color: cgmath::Vector4::new(0.0, 1.0, 1.0, 1.0),
	};
	let instance1 = Instance {
		position: cgmath::Vector3::new(-15.0, 0.0, -30.0),
		rotation: cgmath::Basis3::one(),
		scale: 0.05,
		color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
	};

	let dna = Item {
		file_name: "res/cube.obj".to_string(),
		params: vec![
			(
				instance0,
				Action::Rotate(cgmath::Vector3::new(0.0, 1.0, 0.0), cgmath::Rad(-0.8)),
			),
			(
				instance1,
				Action::Rotate(cgmath::Vector3::new(0.0, 1.0, 0.0), cgmath::Rad(0.8)),
			),
		],
	};
	let elephant = Item {
		file_name: "res/cube.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::FFT,
		)],
	};
	let male = Item {
		file_name: "res/cube.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::FFT,
		)],
	};
	let mushroom = Item {
		file_name: "res/cube.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::FFT,
		)],
	};
	let hammerhead = Item {
		file_name: "res/cube.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::FFT,
		)],
	};
	let flower = Item {
		file_name: "res/cube.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::FFT,
		)],
	};
	*/
	/*
	let eye_background = Item {
		file_name: "res_eye/eye_background.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::Still,
		)],
	};
		*/

	let base = Item {
		file_name: "res/eye_0/base.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::from(Euler {
					x: Deg(0.0),
					y: Deg(0.0),
					z: Deg(0.0),
				}),
				scale: 8.0,
			},
			Action::Still,
		)],
	};

	// eye_lid
	// eye_background
	// eye_ball

	/*
	let eye_lid = Item {
		file_name: "res/eye_0/eyelid.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::from(Euler {
					x: Deg(0.0),
					y: Deg(0.0),
					z: Deg(0.0),
				}),

				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			//Action::Rotate(cgmath::Vector3::new(1.0, 0.0, 0.0), cgmath::Rad(0.8)),
			Action::Still,
		)],
	};
	*/

	/*
	let eye_ball = Item {
		file_name: "res/eye_0/eyeball.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 1.0, 0.0), cgmath::Rad(0.8)),
		)],
	};
	*/

	/*
	let eye_background = Item {
		file_name: "res_eye/eye_background.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::Still,
		)],
	};

	let eye_background = Item {
		file_name: "res_eye/eye_background.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::Still,
		)],
	};

	let eye_background = Item {
		file_name: "res_eye/eye_background.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::one(),
				scale: 8.0,
				color: cgmath::Vector4::new(1.0, 1.0, 1.0, 1.0),
			},
			Action::Still,
		)],
	};
	*/

	pollster::block_on(run(vec![vec![base]]));
}
