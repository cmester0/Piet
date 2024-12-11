pub mod expr;
pub mod optimize;
mod stk_to_file;
mod stk_to_piet;

use expr::{
    parse_expr,
    Expr::{self, *},
};
use pest::*;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fs;

use std::io::Read;

use phf::phf_map;

pub struct PietStackExecutor {
    pub blocks: HashMap<String, Vec<Expr>>,
    pub block_index: HashMap<String, usize>,
    pub stack: Vec<isize>,
    pub label: String,
}

#[derive(Parser)]
#[grammar = "piet_stack/piet_stack.pest"] // relative to src
pub struct PietStackParser;

pub fn parse_string(unparsed: &str) -> (HashMap<String, Vec<Expr>>, HashMap<String, usize>) {
    let document = PietStackParser::parse(Rule::Document, unparsed)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut blocks: HashMap<String, Vec<Expr>> = HashMap::new();
    let mut block_index: HashMap<String, usize> = HashMap::new();

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
                    String::from(name),
                    sub_block.into_inner().map(|x| parse_expr(x)).collect(),
                );
                block_index.insert(String::from(name), bi);
                bi += 1;
            }

            blocks.insert(String::from("term"), vec![]); // TODO
            block_index.insert(String::from("term"), bi); // TODO
        }
        _ => panic!(),
    }

    (blocks, block_index)
}

impl PietStackExecutor {
    fn interpret_expr<I: std::io::Read, O: std::io::Write>(
        &mut self,
        e: Expr,
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
                println!("Debug: {:?}", self.stack);
                false
            }
            Comment(_) => false,
        }
    }

    pub fn interpret<I: std::io::Read, O: std::io::Write>(
        &mut self,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) {
        while self.label != "term" {
            for expr in self.blocks[&self.label].clone() {
                if self.interpret_expr(expr.clone(), input, output) {
                    break;
                }
            }
        }
    }

    pub fn new(filepath: &str) -> Self {
        let unparsed = fs::read_to_string(filepath).expect("cannot read file");
        let (blocks, block_index) = parse_string(unparsed.as_str());
        PietStackExecutor {
            blocks,
            block_index,
            stack: Vec::new(),
            label: String::from("main"),
        }
    }

    pub fn interpret_from_string<I: std::io::Read, O: std::io::Write>(
        unparsed: &str,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) {
        Self::new(unparsed).interpret(input, output);
    }

    pub fn run_on_string(mut self, input: &str) -> String {
        let str_inp: Box<dyn std::io::Read> = Box::new(input.as_bytes());
        let stk_input: std::iter::Peekable<std::io::Bytes<_>> = str_inp.bytes().peekable();

        let mut stk_byt_out = vec![];
        {
            let stk_output: Box<dyn std::io::Write> = Box::new(&mut stk_byt_out);
            self.interpret(&mut Some(stk_input), &mut Some(stk_output));
        }

        String::from_utf8(stk_byt_out).unwrap()
    }
}
