use cgmath::Basis3;
use cgmath::One;
use gseq::run;
use gseq::Action;
use gseq::Instance;
use gseq::Item;
use gseq::FFT;

fn main() {
	let fft = FFT::init(2048, 4, 20, 15000).unwrap();
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

	let item0 = Item {
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
	let item1 = Item {
		file_name: "res/elephant.obj".to_string(),
		params: vec![(
			Instance {
				position: cgmath::Vector3::new(0.0, 0.0, -10.0),
				rotation: Basis3::from(rotation),
				//rotation: Basis3::one(),
				scale: 3.0,
			},
			Action::Rotate(cgmath::Vector3::new(0.0, 1.0, 0.0), cgmath::Rad(-0.8)),
			//Action::Still,
		)],
	};

	pollster::block_on(run(vec![item0, item1]));
	//pollster::block_on(run("res/cube.obj"));
}
