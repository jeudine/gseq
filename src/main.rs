use gseq::run;

fn main() {
	//pollster::block_on(run("../../Downloads/Male.OBJ"));
	pollster::block_on(run("res/cube.obj"));
}
