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
	};
	let instance1 = Instance {
		position: cgmath::Vector3::new(-15.0, 0.0, -30.0),
		rotation: cgmath::Basis3::one(),
		scale: 0.05,
	};

	let dna = Item {
		file_name: "res/DNA.obj".to_string(),
		params: vec![
			(
				instance0,
				Action::Rotate(cgmath::Vector3::new(0.0, 1.0, 0.0), cgmath::Rad(-0.8)),
				//Action::Still,
			),
			(
				instance1,
				//Action::Still,
				Action::Rotate(cgmath::Vector3::new(0.0, 1.0, 0.0), cgmath::Rad(0.8)),
			),
		],
	};
	let rotation = cgmath::Euler {
		x: cgmath::Deg(0.0),
		y: cgmath::Deg(0.0),
		z: cgmath::Deg(0.0),
	};
	let elephant = Item {
		file_name: "res/elephant.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, -4.0, -10.0),
				rotation: Basis3::from(rotation),
				//rotation: Basis3::one(),
				scale: 6.0,
			},
			//Action::Rotate(cgmath::Vector3::new(0.0, 1.0, 0.0), cgmath::Rad(-0.8)),
			//Action::Still,
			Action::FFT,
		)],
	};
	let male = Item {
		file_name: "res/male.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 3.0, -10.0),
				rotation: Basis3::from(rotation),
				//rotation: Basis3::one(),
				scale: 2.0,
			},
			Action::FFT,
			//Action::Still,
		)],
	};

	pollster::block_on(run(vec![dna, elephant, male]));
	//pollster::block_on(run("res/cube.obj"));
}
