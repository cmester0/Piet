use image::open;
use image::DynamicImage;
use mid_smpl_to_stk::SmplToStk;
use piet::advc::advc_to_mid_smpl::*;
use piet::advc::AdvcExecutor;
use piet::mid_smpl::*;
use piet::optimize_stk::StackOptimizer;
use piet::piet_stack::*;
use std::fs::File;
use std::io::{Read, Write};

use piet::util::test_io_string;

pub fn run_piet(img_path: &str, input: &str) -> String {
    test_io_string(input, &mut |read, write| {
        piet::piet::interpret(open(img_path).unwrap(), read, write)
    })
}

pub fn stk_to_piet(filepath: &str, output: &str) {
    // TODO: StackOptimizer should not need input / output !
    let mut optimizer = StackOptimizer::new();
    let img: image::RgbImage = PietStackExecutor::new(filepath).to_png(&mut optimizer);

    let dyn_img = DynamicImage::ImageRgb8(img);

    let _ = dyn_img.save_with_format(output, image::ImageFormat::Png);
}

pub fn smpl_to_stk(filepath: &str, output: &str, registers: usize) {
    let smpl_executor = SmplExecutor::new(filepath, registers); // Default to 0 registers
    let stk_executor = SmplToStk::to_stk(smpl_executor);
    let file_str = stk_executor.to_file_string();

    let mut output_file = File::create(output).unwrap();
    output_file.write(file_str.as_str().as_bytes()).unwrap();
}

pub fn test_stk_vs_piet(path: &str, input: &str, output: &str) {
    let stk_str = PietStackExecutor::new(format!("{}.stk", path).as_str()).run_on_string(input);
    println!("STACK DONE\n");

    assert_eq!(stk_str, output, "STACK FAILED");

    stk_to_piet(
        format!("{}.stk", path).as_str(),
        format!("{}.png", path).as_str(),
    );
    let piet_str = run_piet(format!("{}.png", path).as_str(), input);
    println!("PIET DONE\n");

    assert_eq!(piet_str, output, "PIET FAILED");
}

pub fn test_simpl_vs_stk_vs_piet(path: &str, input: &str, output: &str, registers: usize) {
    // let smpl_str =
    //     SmplExecutor::new(format!("{}.smpl", path).as_str())
    //     .run_on_string(input);
    // println!("STACK DONE\n");

    // assert_eq!(smpl_str, output, "SMPL FAILED");

    smpl_to_stk(
        format!("{}.smpl", path).as_str(),
        format!("{}.stk", path).as_str(),
        registers,
    );

    let stk_str = PietStackExecutor::new(format!("{}.stk", path).as_str()).run_on_string(input);
    println!("STACK DONE\n");

    assert_eq!(stk_str, output, "STACK FAILED");

    stk_to_piet(
        format!("{}.stk", path).as_str(),
        format!("{}.png", path).as_str(),
    );
    let piet_str = run_piet(format!("{}.png", path).as_str(), input);
    println!("PIET DONE\n");

    assert_eq!(piet_str, output, "PIET FAILED");
}

pub fn test_advc_no_file(filepath: &str, input: &str, output: &str, registers: usize) {
    let advc_executor = AdvcExecutor::new(filepath, registers);
    let smpl_executor = AdvcToSmpl::to_smpl(advc_executor);
    let mut stk_executor = SmplToStk::to_stk(smpl_executor);
    stk_executor.optimize();
    let mut optimizer = StackOptimizer::new();
    let img: image::RgbImage = stk_executor.to_png(&mut optimizer);
    let dyn_img: DynamicImage = DynamicImage::ImageRgb8(img);

    assert_eq!(
        output,
        test_io_string(input, &mut |read, write| {
            stk_executor.interpret(read, write);
        }),
        "STACK FAILED"
    );

    println!("{:?}", test_io_string(input, &mut |read, write| {
        piet::piet::interpret(dyn_img.clone(), read, write);
    }));

    assert_eq!(
        output,
        test_io_string(input, &mut |read, write| {
            piet::piet::interpret(dyn_img.clone(), read, write);
        }),
        "PIET FAILED"
    );

}
