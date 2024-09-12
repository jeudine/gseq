use gseq::{run, Show};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let show = if args.len() == 1 {
        Show::Lua
    } else {
        Show::MariusJulien
    };
    pollster::block_on(run(1, show));
}
