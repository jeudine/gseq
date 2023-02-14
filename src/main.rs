use cgmath;
use gseq::run;
use gseq::Action;
use gseq::Instance;
use gseq::Item;

fn main() {
	let instance = Instance {
		position: cgmath::Vector3::new(0.0, 0.0, -20.0),
		rotation: cgmath::Quaternion::new(0.0, 0.0, 0.0, 0.0),
	};
	let item0 = Item {
		file_name: "../../Downloads/DNA.obj".to_string(),
		params: vec![(instance, Action::Still)],
	};
	pollster::block_on(run(vec![item0]));
	//pollster::block_on(run("res/cube.obj"));
}
