mod expr;
pub mod smpl_to_stk;

use expr::*;
use pest::*;
use pest_derive::Parser;
use std::collections::HashMap;

use crate::piet_interpreter::CMD;

#[derive(Clone)]
pub enum Variable {
    NUM(isize),
    LIST(Vec<isize>),
}

impl Variable {
    fn value(self) -> isize {
        match self {
            Variable::NUM(i) => i,
            _ => panic!(),
        }
    }
}

pub struct SmplExecutor {
    pub blocks: HashMap<String, Vec<Expr>>,
    pub block_index: HashMap<String, usize>,
    pub variables: HashMap<String, Variable>,
    pub stack: Vec<isize>,
    pub label: String,
}

#[derive(Parser)]
#[grammar = "smpl/smpl.pest"] // relative to src
pub struct SmplParser;

pub fn parse_string(
    unparsed: &str,
) -> (HashMap<String, Vec<Expr>>, HashMap<String, usize>, HashMap<String, Variable>) {
    let document = SmplParser::parse(Rule::Document, unparsed)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut blocks: HashMap<String, Vec<Expr>> = HashMap::new();
    let mut block_index: HashMap<String, usize> = HashMap::new();
    let mut variables: HashMap<String, Variable> = HashMap::new();

    match document.as_rule() {
        Rule::Document => {
            let mut v: pest::iterators::Pairs<Rule> = document.into_inner();

            let mut pre_main = v.next().unwrap();
            loop {
                match pre_main.as_rule() {
                    Rule::Imports => {
                        // TODO: handle import
                        // println!("{}", "Imports");
                        // variables.insert(_ , _)
                    }
                    Rule::Variables => {
                        let a = pre_main.into_inner();
                        for variable in a {
                            match variable.as_rule() {
                                Rule::Variable => {
                                    let mut var = variable.into_inner();
                                    let name = var.next().unwrap();
                                    let var_type = var.next().unwrap();

                                    match var_type.as_str() {
                                        "num" => {
                                            variables.insert(String::from(name.as_str()), Variable::NUM(0isize));
                                        }
                                        "list" => {
                                            variables
                                                .insert(String::from(name.as_str()), Variable::LIST(Vec::new()));
                                        }
                                        _ => (),
                                    }
                                }
                                _ => todo!("Not variable!"),
                            }
                        }
                    }
                    _ => break,
                }

                pre_main = v.next().unwrap();
            }

            let mut label_map : HashMap<String, String> = HashMap::new();
            let mut label_count = 0;

            let main = pre_main;

            match main.as_rule() {
                Rule::SubBlock => {
                    blocks.insert(String::from("main"), main.into_inner().map(|x| parse_expr(x, &label_map)).collect());
                    block_index.insert(String::from("main"), 0);
                }
                _ => panic!(),
            }

            for b in v {
                let mut block = b.into_inner();
                if block.size_hint().0 == 0 {
                    continue;
                }

                let label = block.next().unwrap();
                let name = match Label::parse_label(label, &label_map) {
                    Label::Name(name) => {
                        blocks.insert(name.clone(), vec![]);
                        block_index.insert(name.clone(), block_index.len());
                        name
                    },
                    Label::Ref(name) => {
                        name
                    },
                };

                let sub_block = block.next().unwrap();

                for x in sub_block.into_inner() {
                    match x.as_rule() {
                        Rule::NewLabel => {
                            let ref_label = String::from(x.into_inner().next().unwrap().as_str());
                            let actual_label = format!("l_ref_{}", label_count);
                            label_count += 1;
                            label_map.insert(ref_label, actual_label.clone());
                            blocks.insert(actual_label.clone(), vec![Expr::Instr(CMD::Nop)]);
                            block_index.insert(actual_label.clone(), block_index.len());
                        },
                        Rule::Expr =>
                            blocks.get_mut(&name).unwrap().push(parse_expr(x, &label_map)),
                        _ => panic!(),
                    }
                }
            }

            blocks.insert(String::from("term"), vec![]); // TODO
            block_index.insert(String::from("term"), block_index.len());
        }
        _ => panic!(),
    }

    (blocks, block_index, variables)
}

impl SmplExecutor {
    fn interpret_expr<I: std::io::Read, O: std::io::Write>(
        &mut self,
        e: Expr,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) -> bool {
        match e {
            Expr::Instr(cmd) => {
                cmd.interpret(&mut self.stack, input, output);
                false
            }
            Expr::Goto(l) => {
                self.label = l.get_label_name();
                true
            }
            Expr::Branch(thn, els) => {
                let a = self.stack.pop().unwrap();
                if a == 0 {
                    self.label = els.get_label_name();
                } else {
                    self.label = thn.get_label_name();
                }
                true
            }
            Expr::Debug => {
                println!("Debug: {:?}", self.stack);
                false
            }
            Expr::Comment(_) => {
                false
            }
            Expr::Eq => {
                let a = self.stack.pop().unwrap();
                let b = self.stack.pop().unwrap();
                self.stack.push((b == a) as isize);
                false
            }
            Expr::Get(s) => {
                self.stack.push(self.variables[&s].clone().value());
                false
            }
            Expr::Set(s) => {
                self.variables
                    .insert(s, Variable::NUM(self.stack.pop().unwrap()));
                false
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
                if self.interpret_expr(expr, input, output) {
                    break;
                }
            }
        }
    }

    pub fn new(
        unparsed: &str,
    ) -> Self {
        let (blocks, block_index, variables) = parse_string(unparsed);
        SmplExecutor {
            blocks,
            block_index,
            variables,
            stack: Vec::new(),
            label: String::from("main"),
        }
    }

    pub fn interpret_from_string<I: std::io::Read, O: std::io::Write>(
        unparsed: &str,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) {
        SmplExecutor::new(unparsed).interpret(input, output);
    }
}
