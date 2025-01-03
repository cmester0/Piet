pub mod optimize;
mod stk_to_file;
mod stk_to_piet;

use crate::optimize_stk::StackOptimizer;
use crate::piet_interpreter::CMD;
use image::DynamicImage;
use num::BigInt;
use pest::iterators::Pair;
use pest::*;
use pest_derive::Parser;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub enum Expr {
    Instr(CMD),
    Goto(String),
    Branch(String, String),
    Debug,
    Comment(String),
    GotoStk,
}
pub use Expr::*;

pub fn parse_expr(e: Pair<Rule>) -> Expr {
    if e.as_rule() != Rule::Expr {
        panic!()
    }
    let mut e = e.into_inner(); // .next().unwrap();
    match e.next().unwrap().as_rule() {
        Rule::Push => Instr(CMD::Push(e.next().unwrap().as_str().parse().unwrap())),
        Rule::Pop => Instr(CMD::Pop),
        Rule::Not => Instr(CMD::Not),
        Rule::Add => Instr(CMD::Add),
        Rule::Greater => Instr(CMD::Greater),
        Rule::Sub => Instr(CMD::Sub),
        Rule::Div => Instr(CMD::Div),
        Rule::Mod => Instr(CMD::Mod),
        Rule::Mul => Instr(CMD::Mul),
        Rule::Dup => Instr(CMD::Dup),
        Rule::InN => Instr(CMD::InN),
        Rule::InC => Instr(CMD::InC),
        Rule::Goto => Goto(String::from(e.next().unwrap().as_str())),
        Rule::Branch => Branch(
            String::from(e.next().unwrap().as_str()),
            String::from(e.next().unwrap().as_str()),
        ),
        Rule::Debug => Debug,
        // TODO: Parse comments and add them into AST?
        Rule::OutC => Instr(CMD::OutC),
        Rule::OutN => Instr(CMD::OutN),
        Rule::Roll => Instr(CMD::Roll),
        Rule::GotoStk => GotoStk,
        _ => panic!("unmatched expression"),
    }
}

pub struct PietStackExecutor {
    pub blocks: HashMap<String, Vec<Expr>>,
    pub block_index: HashMap<String, usize>,
    pub stack: Vec<BigInt>,
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
                if a == 0.into() {
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
            GotoStk => {
                let new_label = self.stack.pop().unwrap();
                let mut success = false;
                for (k, v) in self.block_index.clone() {
                    let label_int: BigInt = v.into();
                    if label_int == new_label {
                        self.label = k;
                        success = true;
                        break;
                    }
                }
                if !success {
                    panic!("No label");
                }
                true
            }
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

    pub fn handle_stk(
        mut self,
        output: Option<String>,
        optimize_stk: bool,
        run_stk: bool,
        to_piet: Option<String>,
        run_piet: bool,
        gui_piet: bool,
        steps_per_frame: usize,
        start_frame: usize,
        skip_whitespace: bool,
    ) {
        if optimize_stk {
            self.optimize()
        }

        if output.is_some() {
            let file_str = self.to_file_string();
            let mut stk_file = File::create(output.clone().unwrap()).unwrap();
            stk_file.write(file_str.as_str().as_bytes()).unwrap();
        }

        if run_stk {
            let input = std::io::stdin().bytes().peekable();
            let output = std::io::stdout();

            self.interpret(&mut Some(input), &mut Some(output));
        }

        if !(to_piet.is_some() || run_piet) {
            return;
        }

        let mut optimizer = StackOptimizer::new();
        let img: image::RgbImage = self.to_png(&mut optimizer);
        let dyn_img = DynamicImage::ImageRgb8(img);

        crate::piet::handle_piet(
            dyn_img,
            to_piet,
            run_piet,
            gui_piet,
            steps_per_frame,
            start_frame,
            skip_whitespace,
        );
    }
}
