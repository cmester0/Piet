use clap::Parser as CliParser;
use image::DynamicImage;
use piet::advc::advc_to_mid_smpl::AdvcToSmpl;
use piet::advc::AdvcExecutor;
use piet::mid_smpl::mid_smpl_to_stk::SmplToStk;
use piet::optimize_stk::StackOptimizer;
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
    optimize_stk: bool,
    #[arg(short, long)]
    run_stk: bool,
    #[arg(short, long)]
    to_piet: Option<String>,
    #[arg(short, long)]
    run_piet: bool,
    #[arg(short, long)]
    registers: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let registers = args.registers.unwrap_or(5);

    let advc_executor = AdvcExecutor::new(args.filepath.as_str(), registers);

    // if args.run_stk {
    //     advc_executor.interpret(
    //         &mut Some(input),
    //         &mut Some(output),
    //     );
    // }

    if args.output.is_some() || args.to_stk.is_some() || args.to_piet.is_some() || args.run_stk {
        let smpl_executor = AdvcToSmpl::to_smpl(advc_executor);

        if args.output.is_some() {
            let file_str = smpl_executor.to_file_string();
            let mut output_file = File::create(args.output.clone().unwrap()).unwrap();
            output_file.write(file_str.as_str().as_bytes()).unwrap();
        }

        if args.to_stk.is_some() || args.to_piet.is_some() || args.run_stk {
            let mut stk_executor = SmplToStk::to_stk(smpl_executor);

            if args.optimize_stk {
                stk_executor.optimize()
            }

            if args.to_stk.is_some() {
                let file_str = stk_executor.to_file_string();
                let mut stk_file = File::create(args.to_stk.clone().unwrap()).unwrap();
                stk_file.write(file_str.as_str().as_bytes()).unwrap();
            }

            if args.to_piet.is_some() || args.run_piet {
                let mut optimizer = StackOptimizer::new();
                let img: image::RgbImage = stk_executor.to_png(&mut optimizer);
                let dyn_img = DynamicImage::ImageRgb8(img);

                if args.to_piet.is_some() {
                    let _ = dyn_img.save_with_format(args.to_piet.clone().unwrap(), image::ImageFormat::Png);
                }

                if args.run_piet {
                    let input = std::io::stdin().bytes().peekable();
                    let output = std::io::stdout();

                    piet::piet::interpret(dyn_img, &mut Some(input), &mut Some(output));
                }
            }

            if args.run_stk {
                let input = std::io::stdin().bytes().peekable();
                let output = std::io::stdout();

                stk_executor.interpret(&mut Some(input), &mut Some(output));
            }
        }
    }
}
