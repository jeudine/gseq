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

	let item0 = Item {
		file_name: "../../Downloads/DNA.obj".to_string(),
		params: vec![
			(
				instance0,
				Action::Rotate(cgmath::Quaternion::new(0.2, 0.0, 0.1, 0.0)),
			),
			(instance1, Action::Still),
		],
	};
	pollster::block_on(run(vec![item0]));
	//pollster::block_on(run("res/cube.obj"));
}
