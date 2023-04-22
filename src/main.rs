use cgmath::Basis3;
use cgmath::One;
use gseq::run;
use gseq::Action;
use gseq::Instance;
use gseq::Item;

fn main() {
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
		file_name: "res/DNA.obj".to_string(),
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
		file_name: "res/elephant.obj".to_string(),
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
		file_name: "res/male.obj".to_string(),
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
		file_name: "res/mushroom.obj".to_string(),
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
		file_name: "res/hammerhead.obj".to_string(),
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
		file_name: "res/flower.obj".to_string(),
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

	pollster::block_on(run(
		vec![dna, elephant, male, mushroom, hammerhead],
		vec![flower],
	));
}
