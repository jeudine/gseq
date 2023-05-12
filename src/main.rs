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

	let pupil_r = Item {
		file_name: "res/eye/pupil.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let pupil_ring_r = Item {
		file_name: "res/eye/pupil_ring.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let outside_r = Item {
		file_name: "res/eye/outside.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::from(Euler {
					x: Deg(0.0),
					y: Deg(0.0),
					z: Deg(0.0),
				}),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let iris1_r = Item {
		file_name: "res/eye/iris1.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let iris2_r = Item {
		file_name: "res/eye/iris2.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let iris3_r = Item {
		file_name: "res/eye/iris3.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let pupil_cage_r = Item {
		file_name: "res/eye/pupil_cage.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let wireframe_r = Item {
		file_name: "res/eye/wireframe.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let ball_cube_r = Item {
		file_name: "res/eye/ball_cube.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 0.7, 0.714), cgmath::Rad(0.3)),
		)],
	};

	let ball_coral_r = Item {
		file_name: "res/eye/ball_coral.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 0.5, 0.866), cgmath::Rad(0.2)),
		)],
	};

	let ball_pikes_r = Item {
		file_name: "res/eye/ball_pikes.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 0.2, 0.98), cgmath::Rad(-0.3)),
		)],
	};

	let ball_pikes2_r = Item {
		file_name: "res/eye/ball_pikes2.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 0.98, 0.2), cgmath::Rad(0.3)),
		)],
	};

	let torus_outside_r = Item {
		file_name: "res/eye/torus_outside.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::from(Euler {
					x: Deg(0.0),
					y: Deg(0.0),
					z: Deg(0.0),
				}),

				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 0.0, 1.0), cgmath::Rad(0.3)),
		)],
	};

	let torus1_r = Item {
		file_name: "res/eye/torus1.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.9055, 0.3, 0.3), cgmath::Rad(0.4)),
		)],
	};

	let torus2_r = Item {
		file_name: "res/eye/torus2.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let torus3_r = Item {
		file_name: "res/eye/torus3.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.2, 0.8944, 0.4), cgmath::Rad(0.1)),
		)],
	};

	let torus_inside_r = Item {
		file_name: "res/eye/torus_inside.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	let torus_inside2_r = Item {
		file_name: "res/eye/torus_inside2.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	pollster::block_on(run(vec![
		vec![
			pupil_r,      //done
			pupil_ring_r, //done
			outside_r,    //done
			iris1_r,
			iris2_r,
			iris3_r,
			//pupil_cage_r,
			//wireframe_r,
			ball_cube_r,     //done
			ball_coral_r,    //done
			ball_pikes_r,    //done
			ball_pikes2_r,   //done
			torus_outside_r, //done
			torus1_r,        //done
			torus2_r,        //done
			torus3_r,        //done
			                 //torus_inside_r,
			                 //torus_inside2_r,
		],
		//vec![],
	]));

	/*
	let cube = Item {
		file_name: "res/cube.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -5.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::FFT,
		)],
	};

	pollster::block_on(run(vec![vec![cube]]));
	*/
}
