use pest::iterators::Pair;

use super::Rule;
use crate::piet_interpreter::CMD;

#[derive(Debug, Clone)]
pub enum Expr {
    Instr(CMD),
    Goto(String),
    Branch(String, String),
    Debug,

    Eq,

    Set(String),
    Get(String),
}
use Expr::*;

pub fn parse_expr(e: Pair<Rule>) -> Expr {
    if e.as_rule() != Rule::Expr {
        panic!()
    }
    let mut e = e.into_inner(); // .next().unwrap();
    match e.next().unwrap().as_rule() {
        Rule::Push => {
            let n = e.next().unwrap();
            match n.as_rule() {
                Rule::Number => {
                    Instr(CMD::Push(n.as_str().parse().unwrap()))
                }
                Rule::Char => {
                    Instr(CMD::Push(n.as_str().chars().next().unwrap() as isize))
                }
                _ => panic!("Trying to push non-number"),
            }
        }
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
        Rule::Branch => Branch(String::from(e.next().unwrap().as_str()), String::from(e.next().unwrap().as_str())),
        Rule::Debug => Debug,
        Rule::OutC => Instr(CMD::OutC),
        Rule::OutN => Instr(CMD::OutN),
        Rule::Roll => Instr(CMD::Roll),

        Rule::Eq => Eq,
        Rule::Set => Set(String::from(e.next().unwrap().as_str())),
        Rule::Get => Get(String::from(e.next().unwrap().as_str())),

        x => panic!("unmatched expression {:?}", x),
    }
}
