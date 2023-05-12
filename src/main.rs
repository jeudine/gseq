use cgmath::Basis3;
use cgmath::One;
use cgmath::{Deg, Euler};
use gseq::run;
use gseq::Action;
use gseq::Instance;
use gseq::Item;

fn main() {
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

	let pupil_l = Item {
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

	let pupil_ring_l = Item {
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

	let outside_l = Item {
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

	let iris2_l = Item {
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

	let iris3_l = Item {
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

	let ball_cube_l = Item {
		file_name: "res/eye/ball_cube.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 0.7, 0.714), cgmath::Rad(-0.3)),
		)],
	};

	let ball_coral_l = Item {
		file_name: "res/eye/ball_coral.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 0.5, 0.866), cgmath::Rad(-0.2)),
		)],
	};

	let ball_pikes_l = Item {
		file_name: "res/eye/ball_pikes.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 0.2, 0.98), cgmath::Rad(0.3)),
		)],
	};

	let ball_pikes2_l = Item {
		file_name: "res/eye/ball_pikes2.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 0.98, 0.2), cgmath::Rad(-0.3)),
		)],
	};

	let torus_outside_l = Item {
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
			Action::Rotate(cgmath::Vector3::new(0.0, 0.0, 1.0), cgmath::Rad(-0.3)),
		)],
	};

	let torus1_l = Item {
		file_name: "res/eye/torus1.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.9055, 0.3, 0.3), cgmath::Rad(-0.4)),
		)],
	};

	let torus2_l = Item {
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

	let torus3_l = Item {
		file_name: "res/eye/torus3.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, 0.0),
				rotation: Basis3::one(),
				scale: 1.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.2, 0.8944, 0.4), cgmath::Rad(-0.1)),
		)],
	};

	pollster::block_on(run(vec![
		vec![
			pupil_r,         //fft
			pupil_ring_r,    //fft
			outside_r,       //fft
			iris2_r,         //fft
			iris3_r,         //fft
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
