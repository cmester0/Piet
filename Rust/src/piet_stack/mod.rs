pub mod expr;

use crate::optimize_stk::StackOptimizer;
use crate::piet_color::*;
use crate::piet_interpreter::*;
use expr::{
    parse_expr,
    Expr::{self, *},
};
use image::Rgb;
use image::RgbImage;
use ndarray::Array;
use ndarray::Ix2;
use pest::*;
use pest_derive::Parser;
use std::cmp;
use std::collections::HashMap;

mod stk_to_piet;

pub struct PietStackExecutor<'a> {
    pub blocks: &'a HashMap<&'a str, Vec<Expr<'a>>>,
    pub block_index: &'a HashMap<&'a str, usize>,
    pub stack: Vec<isize>,
    pub label: &'a str,
}

#[derive(Parser)]
#[grammar = "piet_stack/piet_stack.pest"] // relative to src
pub struct PietStackParser;

pub fn parse_string<'a>(unparsed: &'a str) -> (HashMap<&str, Vec<Expr>>, HashMap<&str, usize>) {
    let document = PietStackParser::parse(Rule::Document, unparsed)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut blocks: HashMap<&str, Vec<Expr>> = HashMap::new();
    let mut block_index: HashMap<&str, usize> = HashMap::new();

    match document.as_rule() {
        Rule::Document => {
            let v: pest::iterators::Pairs<Rule> = document.into_inner();

            let mut bi: usize = 0;

            for b in v {
                let mut block = b.into_inner();
                if block.size_hint().0 == 0 {
                    continue;
                }
                let name = block.next().unwrap().as_str();
                let sub_block = block.next().unwrap();
                blocks.insert(
                    name,
                    sub_block.into_inner().map(|x| parse_expr(x)).collect(),
                );
                block_index.insert(name, bi);
                bi += 1;
            }

            blocks.insert("term", vec![]); // TODO
            block_index.insert("term", bi); // TODO
        }
        _ => panic!(),
    }

    (blocks, block_index)
}

impl<'a> PietStackExecutor<'a> {
    fn interpret_expr<I: std::io::Read, O: std::io::Write>(
        &mut self,
        e: &Expr<'a>,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) -> bool {
        match e {
            Instr(cmd) => {
                cmd.interpret(&mut self.stack, input, output);
                false
            }
            Goto(l) => {
                self.label = l;
                true
            }
            Branch(thn, els) => {
                let a = self.stack.pop().unwrap();
                if a == 0 {
                    self.label = els;
                } else {
                    self.label = thn;
                }
                true
            }
            Debug => {
                println!("Debug: ");
                false
            }
        }
    }

    pub fn interpret<I: std::io::Read, O: std::io::Write>(
        &mut self,
        input: &'a mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &'a mut Option<O>,
    ) {
        while self.label != "term" {
            for expr in &self.blocks[self.label] {
                if self.interpret_expr(expr, input, output) {
                    break;
                }
            }
        }
    }

    pub fn interpret_from_string<I: std::io::Read, O: std::io::Write>(
        unparsed: &'a str,
        input: &'a mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &'a mut Option<O>,
    ) {
        let (blocks, block_index) = parse_string(unparsed);
        let mut executor: PietStackExecutor = PietStackExecutor {
            blocks: &blocks,
            block_index: &block_index,
            stack: Vec::new(),
            label: "main",
        };
        executor.interpret(input, output);
    }
}
