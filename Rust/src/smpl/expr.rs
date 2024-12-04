use pest::iterators::Pair;

use super::Rule;
use crate::piet_interpreter::CMD;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Label {
    Name(String),
    Ref(String),
}

impl Label {
    pub fn parse_label(e: Pair<Rule>, label_map: &HashMap<String, String>) -> Label {
        if e.as_rule() != Rule::Label {
            panic!()
        }
        let n = e.into_inner().next().unwrap();
        match n.as_rule() {
            Rule::LabelName => {
                let name = n.as_str();
                Label::Name(String::from(name))
            }
            Rule::LabelRef => {
                let label_ref: &str = n.into_inner().next().unwrap().as_str();
                let name = label_map[&String::from(label_ref)].clone();
                Label::Ref(String::from(name))
            }
            _ => panic!(),
        }
    }

    pub fn get_label_name(self) -> String {
        match self {
            Label::Name(n) | Label::Ref(n) => n,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Instr(CMD),
    Goto(Label),
    Branch(Label, Label),
    Debug,

    // Lib(String),
    Eq,
    Set(String),
    Get(String),

    Comment(String),
}
use Expr::*;

pub fn parse_expr(e: Pair<Rule>, label_map: &HashMap<String, String>) -> Expr {
    if e.as_rule() != Rule::Expr {
        panic!()
    }
    let mut e = e.into_inner(); // .next().unwrap();
    match e.next().unwrap().as_rule() {
        Rule::Push => {
            let n = e.next().unwrap();
            match n.as_rule() {
                Rule::Number => Instr(CMD::Push(n.as_str().parse().unwrap())),
                Rule::Char => Instr(CMD::Push(n.as_str().chars().next().unwrap() as isize)),
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
        Rule::Goto => Goto(Label::parse_label(e.next().unwrap(), label_map)),
        Rule::Branch => Branch(
            Label::parse_label(e.next().unwrap(), label_map),
            Label::parse_label(e.next().unwrap(), label_map),
        ),
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
