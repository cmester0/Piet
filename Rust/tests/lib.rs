use image::DynamicImage;
use piet::optimize_stk::StackOptimizer;
use piet::piet::*;
use piet::piet_stack::*;
use std::fs;
use std::io::Read;

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
    let img: image::RgbImage = PietStackExecutor::to_png(&mut optimizer, unparsed_file.as_str());

    let dyn_img = DynamicImage::ImageRgb8(img);

    let _ = dyn_img.save_with_format(output, image::ImageFormat::Png);
}

pub fn run_piet(img_path: &str, input: &str) -> String {
    let str_inp: Box<dyn std::io::Read> = Box::new("".as_bytes());
    let piet_input: std::iter::Peekable<std::io::Bytes<_>> = str_inp.bytes().peekable();

    let mut piet_byt_out = vec![];
    {
        let piet_output: Box<dyn std::io::Write> = Box::new(&mut piet_byt_out);
        piet::piet::interpret(img_path, &mut Some(piet_input), &mut Some(piet_output));
    }

    String::from_utf8(piet_byt_out).unwrap()
}

pub fn test_stk_vs_piet(path: &str, input: &str, output: &str) {
    let stk_str = run_stk(format!("{}.stk", path).as_str(), input);

    assert_eq!(stk_str, output);

    stk_to_piet(
        format!("{}.stk", path).as_str(),
        format!("{}.png", path).as_str(),
    );
    let piet_str = run_piet(format!("{}.png", path).as_str(), input);

    assert_eq!(piet_str, output);
}

#[test]
pub fn test_set_var_get_var() {
    test_stk_vs_piet("./tests/test_set_var_get_var", "", "42");
}

// #[test]
// pub fn test_length() {
//     test_stk_vs_piet("./tests/test_length", "", "7\n");
// }

#[test]
pub fn test_eq0() {
    test_stk_vs_piet("./tests/test_eq0", "", "0\n");
}

#[test]
pub fn test_eq1() {
    test_stk_vs_piet("./tests/test_eq1", "", "1\n");
}

// #[test]
// pub fn test_append() {
//     test_stk_vs_piet("./tests/test_append", "", "[114,119,113,115,110,]\n");
// }

// #[test]
// pub fn test_print_listC() {
//     test_stk_vs_piet("./tests/test_print_listC", "", "hello world\n");
// }

// #[test]
// pub fn test_readlines() {
//     test_stk_vs_piet("./tests/test_readlines",
//                      "hello\nworld how\nare you\ndoing",
//                      "hello\nworld how\nare you\ndoing");
// }

// #[test]
// pub fn test_swap() {
//     test_stk_vs_piet("./tests/test_swap", "23 43", "43 23");
// }

// #[test]
// pub fn test_set_elem_get_elem() {
//     test_stk_vs_piet("./tests/test_set_elem_get_elem", "", "14\n11\n");
// }

#[test]
pub fn test_dup_at_depth() {
    test_stk_vs_piet("./tests/test_dup_at_depth", "", "43\n");
}

// #[test]
// pub fn test_in() {
//     test_stk_vs_piet("./tests/test_in", "", "10\n");
// }

// #[test]
// pub fn test_in_tuple() {
//     test_stk_vs_piet("./tests/test_in_tuple", "", "10\n");
// }
