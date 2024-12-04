use clap::Parser as CliParser;
use image::DynamicImage;
use piet::optimize_stk::StackOptimizer;
use piet::piet_stack::*;
use std::fs;
use std::io::Read;

#[derive(CliParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long)]
    run: bool,
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    let unparsed_file = fs::read_to_string(args.filepath).expect("cannot read file");

    let input = std::io::stdin().bytes().peekable();
    let output = std::io::stdout();

    let mut stk_executor = PietStackExecutor::new(unparsed_file.as_str());

    if args.run {
        stk_executor.interpret::<std::io::Stdin, std::io::Stdout>(
            &mut Some(input),
            &mut Some(output),
        );
    }

    if args.output.is_some() {
        let mut optimizer = StackOptimizer::new();
        let img: image::RgbImage = stk_executor.to_png(&mut optimizer);
        let dyn_img = DynamicImage::ImageRgb8(img);
        let _ = dyn_img.save_with_format(args.output.unwrap(), image::ImageFormat::Png);
    }
}
