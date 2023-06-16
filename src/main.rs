use cgmath::Basis3;
use cgmath::One;
use cgmath::{Deg, Euler};
use gseq::run;
use gseq::Instance;
use gseq::Item;

fn main() {
	let outside_r = Item {
		file_name: "res/eye2/outside.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::from(Euler {
				x: Deg(0.0),
				y: Deg(0.0),
				z: Deg(0.0),
			}),
			scale: 1.0,
		},
	};

	let outside_copie_r = Item {
		file_name: "res/eye2/outside_copie.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::from(Euler {
				x: Deg(0.0),
				y: Deg(0.0),
				z: Deg(0.0),
			}),
			scale: 1.0,
		},
	};

	let wireframe_r = Item {
		file_name: "res/eye2/wireframe.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::from(Euler {
				x: Deg(0.0),
				y: Deg(0.0),
				z: Deg(0.0),
			}),
			scale: 1.0,
		},
	};

	let iris_3_r = Item {
		file_name: "res/eye2/iris_3.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::from(Euler {
				x: Deg(0.0),
				y: Deg(0.0),
				z: Deg(0.0),
			}),
			scale: 1.0,
		},
	};

	let pupil_r = Item {
		file_name: "res/eye2/pupil.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let outside_ball_coral_r = Item {
		file_name: "res/eye2/outside_ball_coral.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let outside_ball_coral_1_r = Item {
		file_name: "res/eye2/outside_ball_coral_1.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let outside_ball_cube_r = Item {
		file_name: "res/eye2/outside_ball_cube.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let outside_ball_cube_1_r = Item {
		file_name: "res/eye2/outside_ball_cube_1.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let outside_ball_piek_r = Item {
		file_name: "res/eye2/outside_ball_piek.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let outside_ball_piek_1_r = Item {
		file_name: "res/eye2/outside_ball_piek_1.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let outside_ball_piek_2_r = Item {
		file_name: "res/eye2/outside_ball_piek_2.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let outside_ball_piek_3_r = Item {
		file_name: "res/eye2/outside_ball_piek_3.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let torus_1_r = Item {
		file_name: "res/eye2/torus_1.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let torus_2_r = Item {
		file_name: "res/eye2/torus_2.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	let torus_outside_r = Item {
		file_name: "res/eye2/torus_outside.obj".to_string(),
		instance: Instance {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: Basis3::one(),
			scale: 1.0,
		},
	};

	pollster::block_on(run(vec![
		vec![
			outside_r,
			outside_copie_r,
			wireframe_r,
			iris_3_r,
			pupil_r,
			outside_ball_coral_r,
			outside_ball_coral_1_r,
			outside_ball_cube_r,
			outside_ball_cube_1_r,
			outside_ball_piek_r,
			outside_ball_piek_1_r,
			outside_ball_piek_2_r,
			outside_ball_piek_3_r,
			torus_1_r,
			torus_2_r,
			torus_outside_r,
		],
		/*
		vec![
			pupil_l,         //fft
			pupil_ring_l,    //fft
			outside_l,       //fft
			iris2_l,         //fft
			iris3_l,         //fft
			ball_cube_l,     //done
			ball_coral_l,    //done
			ball_pikes_l,    //done
			ball_pikes2_l,   //done
			torus_outside_l, //done
			torus1_l,        //done
			torus2_l,        //done
			torus3_l,        //done
							 //torus_inside_r,
							 //torus_inside2_r,
		],
		*/
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
