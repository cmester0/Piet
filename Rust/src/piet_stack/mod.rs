pub mod expr;
use expr::{
    parse_expr,
    Expr::{self, *},
};

use pest::*;
use pest_derive::Parser;

use std::collections::HashMap;

pub struct PietStackExecutor<'a> {
    pub blocks: &'a HashMap<&'a str, Vec<Expr<'a>>>,
    pub stack: Vec<isize>,
    pub label: &'a str,
}

#[derive(Parser)]
#[grammar = "piet_stack/piet_stack.pest"] // relative to src
pub struct PietStackParser;

pub fn parse_string<'a>(unparsed: &'a str) -> HashMap<&str, Vec<Expr>> {
    let document = PietStackParser::parse(Rule::Document, unparsed)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut blocks: HashMap<&str, Vec<Expr>> = HashMap::new();

    match document.as_rule() {
        Rule::Document => {
            let mut v: pest::iterators::Pairs<Rule> = document.into_inner();
            let main = v.next().unwrap();

            match main.as_rule() {
                Rule::SubBlock => {
                    blocks.insert("main", main.into_inner().map(|x| parse_expr(x)).collect());
                }
                _ => panic!(),
            }

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
            }

            blocks.insert("term", vec![]); // TODO
        }
        _ => panic!(),
    }

    blocks
}

impl<'a> PietStackExecutor<'a> {
    fn interpret_expr(
        &mut self,
        e: &Expr<'a>,
        input: &mut std::iter::Peekable<std::io::Bytes<std::io::Stdin>>,
    ) -> bool {
        match e {
            Instr(cmd) => {
                cmd.interpret(&mut self.stack, input);
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

    pub fn interpret(
        &mut self,
        input: &'a mut std::iter::Peekable<std::io::Bytes<std::io::Stdin>>,
    ) {
        while self.label != "term" {
            for expr in &self.blocks[self.label] {
                if self.interpret_expr(expr, input) {
                    break;
                }
            }
        }
    }

    pub fn interpret_from_string(
        unparsed: &'a str,
        input: &'a mut std::iter::Peekable<std::io::Bytes<std::io::Stdin>>,
    ) {
        let mut blocks = parse_string(unparsed);
        let mut executor: PietStackExecutor = PietStackExecutor {
            blocks: &blocks,
            stack: Vec::new(),
            label: "main",
        };
        executor.interpret(input);
    }
}
