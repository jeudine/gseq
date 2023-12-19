use gseq::run;

fn main() {
	pollster::block_on(run(1));
}
