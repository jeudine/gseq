use gseq::run;

fn main() {
	pollster::block_on(run("res/cube.obj"));
}
