use clap::Parser as CliParser;

use pest::*;
use pest_derive::Parser;

use std::fs;
use std::io::Read;
use std::collections::HashMap;

use piet::piet_interpreter::*;

#[derive(Parser)]
#[grammar = "bin/piet_stack.pest"] // relative to src
pub struct MyParser;

pub enum VarType { Num , List }

#[derive(Debug, Copy, Clone)]
pub enum Expr<'a> {
    Instr(CMD),
    Goto(&'a str),
    Branch(&'a str, &'a str),
    Debug,
}
use Expr::*;

#[derive(CliParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
}

fn parse_expr(e: pest::iterators::Pair<Rule>) -> Expr
{
    if e.as_rule() != Rule::Expr { panic!() }
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
        Rule::Goto => Goto(e.next().unwrap().as_str()),
        Rule::Branch => Branch(e.next().unwrap().as_str(), e.next().unwrap().as_str()),
        Rule::Debug => Debug,
        Rule::OutC => Instr(CMD::OutC),
        Rule::OutN => Instr(CMD::OutN),
        Rule::Roll => Instr(CMD::Roll),
        _ => panic!("unmatched expression"),
    }
}

struct PietStackExecutor<'a> {
    blocks: &'a HashMap<&'a str, Vec<Expr<'a>>>,
    stack: Vec<isize>,
    label: &'a str,
}

impl<'a> PietStackExecutor<'a> {
    fn interpret_expr(&mut self, e: &Expr<'a>, input: &mut std::iter::Peekable<std::io::Bytes<std::io::Stdin>>) -> bool {
        match e {
            Instr(cmd) => {
                cmd.interpret(&mut self.stack, input);
                false
            },
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
            },
            Debug => {
                println!("Debug: ");
                false
            },
        }
    }

    fn interpret(&mut self, input: &mut std::iter::Peekable<std::io::Bytes<std::io::Stdin>>) {
        while self.label != "term" {
            for expr in &self.blocks[self.label] {
                if self.interpret_expr(expr, input) { break }
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let unparsed_file = fs::read_to_string(args.filepath).expect("cannot read file");
    let file = MyParser::parse(Rule::Document, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut blocks : HashMap<&str, Vec<Expr>> = HashMap::new();

    match file.as_rule() {
        Rule::Document => {
            let mut v: pest::iterators::Pairs<Rule> = file.into_inner();
            let main = v.next().unwrap();

            match main.as_rule() {
                Rule::SubBlock => {
                    blocks.insert("main", main.into_inner().map(|x| parse_expr(x)).collect());
                },
                _ => panic!(),
            }

            for b in v {
                let mut block = b.into_inner();
                if block.size_hint().0 == 0 { continue; }
                let name = block.next().unwrap().as_str();
                let sub_block = block.next().unwrap();
                blocks.insert(name, sub_block.into_inner().map(|x| parse_expr(x)).collect());
            }

            blocks.insert("term", vec![]); // TODO
        }
        _ => panic!(),
    }

    let mut executor = PietStackExecutor { blocks: &blocks, stack: Vec::new(), label: "main" };
    let mut input = std::io::stdin().bytes().peekable();
    executor.interpret(&mut input);
}
