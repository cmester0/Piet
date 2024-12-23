use clap::Parser;
use image::open;
use piet::piet::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long)]
    gui: bool,
    #[arg(short, long)]
    steps_per_frame: Option<usize>,
    #[arg(short, long)]
    start_frame: Option<usize>,
    #[arg(short, long, action)]
    skip_whitespace: bool,
}

fn main() {
    let args = Args::parse();
    let steps_per_frame = args.steps_per_frame.unwrap_or(1);
    let start_frame = args.steps_per_frame.unwrap_or(0);

    handle_piet(
        open(args.filepath).unwrap(),
        None,
        true,
        args.gui,
        steps_per_frame,
        start_frame,
        args.skip_whitespace,
    );
}
