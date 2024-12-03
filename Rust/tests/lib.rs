use image::DynamicImage;
use piet::optimize_stk::StackOptimizer;
use piet::piet::*;
use piet::piet_stack::*;
use std::fs;
use std::io::Read;

pub fn run_stk(filepath : &str, input: &str) -> String {
    println!("START");
    let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");

    let str_inp : Box<dyn std::io::Read> = Box::new(input.as_bytes());
    let stk_input : std::iter::Peekable<std::io::Bytes<_>> = str_inp.bytes().peekable();

    println!("RUN STK");
    let mut stk_byt_out = vec![];
    {
        let stk_output : Box<dyn std::io::Write> = Box::new ( &mut stk_byt_out );
        PietStackExecutor::interpret_from_string(unparsed_file.as_str(), &mut Some(stk_input), &mut Some(stk_output));
    }

    String::from_utf8(stk_byt_out).unwrap()
}

pub fn stk_to_piet(filepath : &str, output : &str) {
    let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");

    // TODO: StackOptimizer should not need input / output !
    let mut optimizer = StackOptimizer::new();
    let img: image::RgbImage =
        PietStackExecutor::to_png(&mut optimizer, unparsed_file.as_str());

    let dyn_img = DynamicImage::ImageRgb8(img);

    let _ = dyn_img.save_with_format(output, image::ImageFormat::Png);
}

pub fn run_piet(img_path : &str, input: &str) -> String {
    // let img_path : &str = "./tests/test_eq0.png";

    println!("RUN PIET");
    let str_inp : Box<dyn std::io::Read> = Box::new("".as_bytes());
    let piet_input : std::iter::Peekable<std::io::Bytes<_>> = str_inp.bytes().peekable();

    let mut piet_byt_out = vec![];
    {
        let piet_output : Box<dyn std::io::Write> = Box::new ( &mut piet_byt_out );
        piet::piet::interpret(img_path, &mut Some(piet_input), &mut Some(piet_output));
    }

    String::from_utf8(piet_byt_out).unwrap()
}

#[test]
pub fn test_stk_vs_gen() {
    println!("START");

    let filepath : &str = "./tests/test_eq0.stk";
    let img_path : &str = "./tests/test_eq0.png";

    let stk_str = run_stk(filepath, "");
    stk_to_piet(filepath, img_path);
    let piet_str = run_piet(img_path, "");

    println!(">{} vs {}<", stk_str, piet_str);
    assert_eq!(stk_str, piet_str)
}
