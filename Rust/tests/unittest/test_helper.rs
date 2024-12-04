use image::DynamicImage;
use piet::optimize_stk::StackOptimizer;
use piet::piet_stack::*;
use piet::smpl::*;
use smpl_to_stk::SmplToStk;
use std::fs::{self, File};
use std::io::{Read, Write};

pub fn run_piet(img_path: &str, input: &str) -> String {
    let str_inp: Box<dyn std::io::Read> = Box::new(input.as_bytes());
    let piet_input: std::iter::Peekable<std::io::Bytes<_>> = str_inp.bytes().peekable();

    let mut piet_byt_out = vec![];
    {
        let piet_output: Box<dyn std::io::Write> = Box::new(&mut piet_byt_out);
        piet::piet::interpret(img_path, &mut Some(piet_input), &mut Some(piet_output));
    }

    String::from_utf8(piet_byt_out).unwrap()
}

pub fn run_stk(filepath: &str, input: &str) -> String {
    let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");

    let str_inp: Box<dyn std::io::Read> = Box::new(input.as_bytes());
    let stk_input: std::iter::Peekable<std::io::Bytes<_>> = str_inp.bytes().peekable();

    let mut stk_byt_out = vec![];
    {
        let stk_output: Box<dyn std::io::Write> = Box::new(&mut stk_byt_out);
        PietStackExecutor::interpret_from_string(
            unparsed_file.as_str(),
            &mut Some(stk_input),
            &mut Some(stk_output),
        );
    }

    String::from_utf8(stk_byt_out).unwrap()
}

pub fn stk_to_piet(filepath: &str, output: &str) {
    let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");

    // TODO: StackOptimizer should not need input / output !
    let mut optimizer = StackOptimizer::new();
    let img: image::RgbImage = PietStackExecutor::new(unparsed_file.as_str()).to_png(&mut optimizer);

    let dyn_img = DynamicImage::ImageRgb8(img);

    let _ = dyn_img.save_with_format(output, image::ImageFormat::Png);
}

pub fn run_smpl(filepath: &str, input: &str) -> String {
    let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");

    let str_inp: Box<dyn std::io::Read> = Box::new(input.as_bytes());
    let stk_input: std::iter::Peekable<std::io::Bytes<_>> = str_inp.bytes().peekable();

    let mut stk_byt_out = vec![];
    {
        let stk_output: Box<dyn std::io::Write> = Box::new(&mut stk_byt_out);
        SmplExecutor::interpret_from_string(
            unparsed_file.as_str(),
            &mut Some(stk_input),
            &mut Some(stk_output),
        );
    }

    String::from_utf8(stk_byt_out).unwrap()
}

pub fn smpl_to_stk(filepath: &str, output: &str) {
    let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");

    let smpl_executor = SmplExecutor::new(unparsed_file.as_str());
    let stk_executor = SmplToStk::to_stk(smpl_executor);
    let file_str = stk_executor.to_file_string();

    let mut output_file = File::create(output).unwrap();
    output_file.write(file_str.as_str().as_bytes()).unwrap();
}

pub fn test_stk_vs_piet(path: &str, input: &str, output: &str) {
    let stk_str = run_stk(format!("{}.stk", path).as_str(), input);
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

pub fn test_simpl_vs_stk_vs_piet(path: &str, input: &str, output: &str) {
    let smpl_str = run_smpl(format!("{}.smpl", path).as_str(), input);
    println!("STACK DONE\n");

    assert_eq!(smpl_str, output, "SMPL FAILED");

    smpl_to_stk(
        format!("{}.smpl", path).as_str(),
        format!("{}.stk", path).as_str(),
    );

    let stk_str = run_stk(format!("{}.stk", path).as_str(), input);
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
