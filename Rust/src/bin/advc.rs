use clap::Parser as CliParser;
use image::DynamicImage;
use piet::optimize_stk::StackOptimizer;
use piet::advc::AdvcExecutor;
use piet::advc::advc_to_mid_smpl::AdvcToSmpl;
use std::fs::File;
use std::io::{Read, Write};

#[derive(CliParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long)]
    run: bool,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    to_stk: Option<String>,
    #[arg(short, long)]
    to_piet: Option<String>,
    #[arg(short, long)]
    registers: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let input = std::io::stdin().bytes().peekable();
    let output = std::io::stdout();

    let registers = args.registers.unwrap_or(5);

    let mut advc_executor = AdvcExecutor::new(args.filepath.as_str(), registers);

    // if args.run {
    //     advc_executor.interpret(
    //         &mut Some(input),
    //         &mut Some(output),
    //     );
    // }

    if args.output.is_some() || args.to_stk.is_some() || args.to_piet.is_some() {
        let smpl_executor = AdvcToSmpl::to_smpl(advc_executor);

        if args.output.is_some() {
            let file_str = smpl_executor.to_file_string();
            let mut output_file = File::create(args.output.clone().unwrap()).unwrap();
            output_file.write(file_str.as_str().as_bytes()).unwrap();
        }

        // if args.output.is_some() {
        //     let smpl_executor = AdvcToSmpl::to_smpl(advc_executor);
        //     let file_str = smpl_executor.to_file_string();
        //     let mut output_file = File::create(args.output.clone().unwrap()).unwrap();
        //     output_file.write(file_str.as_str().as_bytes()).unwrap();

        //     if args.to_piet.is_some() {
        //         let mut optimizer = StackOptimizer::new();
        //         let img: image::RgbImage = stk_executor.to_png(&mut optimizer);
        //         let dyn_img = DynamicImage::ImageRgb8(img);
        //         let _ = dyn_img.save_with_format(args.output.unwrap(), image::ImageFormat::Png);
        //     }
        // }
    }

}
