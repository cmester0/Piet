use super::{
    parse_expr,
    Expr::{self, *},
};
use crate::optimize_stk::StackOptimizer;
use crate::piet_color::*;
use crate::piet_interpreter::CMD::{self, *};
use crate::piet_interpreter::*;
use image::Rgb;
use image::RgbImage;
use itertools::Itertools;
use ndarray::Array;
use ndarray::Ix2;
use std::cmp;
use std::collections::HashMap;
use std::io::{Read, Write};

// pub struct PietStackExecutor {
//     pub blocks: HashMap<String, Vec<Expr>>,
//     pub block_index: HashMap<String, usize>,
//     pub stack: Vec<isize>,
//     pub label: String,
// }

impl super::PietStackExecutor {
    pub fn to_file_string(&self) -> String {
        let mut stk_byt_out = vec![];
        {
            let mut stk_output: Box<dyn std::io::Write> = Box::new(&mut stk_byt_out);

            for (k, _) in self.block_index.clone().into_iter().sorted_by(|(_, v1), (_, v2)| v1.cmp(v2)) {
                writeln!(stk_output, "label {}", k.clone()).unwrap();
                for e in self.blocks[&k].clone() {
                    match e {
                        Instr(c) => match c {
                            Nop => {}

                            Push(i) => writeln!(stk_output, "push {}", i).unwrap(),
                            Pop => writeln!(stk_output, "pop").unwrap(),
                            Add => writeln!(stk_output, "add").unwrap(),
                            Sub => writeln!(stk_output, "sub").unwrap(),
                            Mul => writeln!(stk_output, "mul").unwrap(),
                            Div => writeln!(stk_output, "div").unwrap(),
                            Mod => writeln!(stk_output, "mod").unwrap(),
                            Not => writeln!(stk_output, "not").unwrap(),
                            Greater => writeln!(stk_output, "greater").unwrap(),
                            Dup => writeln!(stk_output, "dup").unwrap(),
                            Roll => writeln!(stk_output, "roll").unwrap(),
                            InN => writeln!(stk_output, "inN").unwrap(),
                            InC => writeln!(stk_output, "inC").unwrap(),
                            OutN => writeln!(stk_output, "outN").unwrap(),
                            OutC => writeln!(stk_output, "outC").unwrap(),

                            Pointer | Switch => panic!(),
                        },
                        Goto(s) => writeln!(stk_output, "goto {}", s).unwrap(),
                        Branch(a, b) => writeln!(stk_output, "branch {} {}", a, b).unwrap(),
                        Debug => writeln!(stk_output, "debug").unwrap(),
                        Comment(s) => writeln!(stk_output, "# {}", s).unwrap(),
                    }
                }
            }
        }
        String::from_utf8(stk_byt_out).unwrap()
    }
}