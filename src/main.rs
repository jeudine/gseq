use clap::Parser;
use gseq::{run, Show};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Displayed visual show
    #[arg(short, long, default_value_t = Show::MariusJulien)]
    show: Show,
}

fn main() {
    let args = Args::parse();
    pollster::block_on(run(1, args.show));
}
