pub mod expr;
pub mod mid_smpl_to_stk;
pub mod mid_smpl_to_file;

use expr::*;
use pest::*;
use pest_derive::Parser;
use std::collections::HashMap;

use crate::piet_interpreter::CMD;

use pest::iterators::Pair;
use std::fs;

#[allow(unused_imports)]
use std::io::Read;


#[derive(Clone, Debug)]
// #[repr(isize)]
pub enum VariableType {
    NUM = 0,
    LIST = -1,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Variable {
    pub(crate) var_type: VariableType,
    pub(crate) value: isize,
    pub(crate) var_index: usize,
}

#[derive(Clone)]
pub struct SmplExecutor {
    pub blocks: HashMap<String, Vec<Expr>>,
    pub block_index: HashMap<String, usize>,
    pub variables: HashMap<String, Variable>,
    pub stack: Vec<isize>,
    pub label: String,
    pub registers: usize,
    pub imports: HashMap<String, String>,
}

#[derive(Parser)]
#[grammar = "mid_smpl/mid_smpl.pest"] // relative to src
pub struct SmplParser;

pub fn parse_subblocks(
    mut block_name: String,
    sub_block: Pair<Rule>,
    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    variables: &mut HashMap<String, Variable>,

    label_map: &mut HashMap<String, String>,
    label_count: &mut usize,
    imports: &HashMap<String, String>,

    top_level: bool,
) -> String {
    if sub_block.as_rule() != Rule::SubBlock {
        panic!()
    }

    for x in sub_block.into_inner() {
        match x.as_rule() {
            Rule::NewLabel => {
                let ref_label = String::from(x.into_inner().next().unwrap().as_str());
                let actual_label = format!("l_ref_{}", label_count);
                *label_count += 1;
                label_map.insert(ref_label, actual_label.clone());
                blocks.insert(actual_label.clone(), vec![Expr::Instr(CMD::Nop)]);
                block_index.insert(actual_label.clone(), block_index.len());
            }
            Rule::LibFun => {
                let lib_name = String::from(x.into_inner().next().unwrap().as_str());
                if !imports.contains_key(&lib_name) {
                    panic!("could not find key {}", lib_name);
                }
                let s = imports[&lib_name].clone();
                let actual_filepath = s;

                if top_level {
                    blocks
                        .get_mut(&block_name)
                        .unwrap()
                        .push(Expr::Comment(format!("+{}", lib_name)));
                }

                let unparsed_file = fs::read_to_string(actual_filepath.clone())
                    .expect(format!("cannot read file {}", actual_filepath).as_str());

                let mut v = SmplParser::parse(Rule::LibBlocks, &unparsed_file)
                    .expect("unsuccessful parse")
                    .next()
                    .unwrap()
                    .into_inner();

                let initial_sub_block = v.next().unwrap();

                match initial_sub_block.as_rule() {
                    Rule::SubBlock => {
                        block_name = parse_subblocks(
                            block_name.clone(),
                            initial_sub_block,
                            blocks,
                            block_index,
                            variables,
                            label_map,
                            label_count,
                            imports,
                            false,
                        );
                    }
                    _ => panic!("INITIAL not subblock?"),
                }

                let rule_blocks = v.next().unwrap();
                if rule_blocks.as_rule() != Rule::Blocks {
                    panic!("NOT BLOCKS {:?}", v)
                }

                for b in rule_blocks.into_inner() {
                    if let Rule::EOI = b.as_rule() {
                        break;
                    }

                    if b.as_rule() != Rule::Block {
                        panic!()
                    }

                    block_name = parse_block(
                        b,
                        blocks,
                        block_index,
                        variables,
                        label_map,
                        label_count,
                        imports,
                        false,
                    );
                }

                if top_level {
                    blocks
                        .get_mut(&block_name)
                        .unwrap()
                        .push(Expr::Comment(format!("-{}", lib_name)));
                }
            }
            Rule::Expr => blocks
                .get_mut(&block_name)
                .unwrap()
                .push(parse_expr(x, &label_map)),
            _ => panic!(),
        }
    }

    block_name
}

pub fn parse_block(
    b: Pair<Rule>,
    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    variables: &mut HashMap<String, Variable>,

    label_map: &mut HashMap<String, String>,
    label_count: &mut usize,
    imports: &HashMap<String, String>,
    top_level: bool,
) -> String {
    if b.as_rule() != Rule::Block {
        panic!();
    }

    let mut block = b.into_inner();

    let label = block.next().unwrap();
    let name = match Label::parse_label(label, &label_map) {
        Label::Name(name) => {
            blocks.insert(name.clone(), vec![]);
            block_index.insert(name.clone(), block_index.len());
            name
        }
        Label::Ref(name) => name,
    };

    let sub_block = block.next().unwrap();
    parse_subblocks(
        name,
        sub_block,
        blocks,
        block_index,
        variables,
        label_map,
        label_count,
        imports,
        top_level,
    )
}

pub fn parse_string(
    filepath: &str,
    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    variables: &mut HashMap<String, Variable>,
    imports: &mut HashMap<String, String>,
) {
    let unparsed =
        fs::read_to_string(filepath).expect(format!("cannot read file: {}", filepath).as_str());
    let document = SmplParser::parse(Rule::Document, unparsed.as_str())
        .expect(format!("unsuccessful parse of {}", filepath).as_str())
        .next()
        .unwrap();

    match document.as_rule() {
        Rule::Document => {
            let mut v: pest::iterators::Pairs<Rule> = document.into_inner();

            let mut pre_main = v.next().unwrap();
            loop {
                match pre_main.as_rule() {
                    Rule::Imports => {
                        // TODO: handle import
                        let a = pre_main.into_inner();
                        for import in a {
                            match import.as_rule() {
                                Rule::Import => {
                                    let mut imp = import.into_inner();
                                    let name = imp.next().unwrap().as_str();
                                    let filepath = imp.next().unwrap().as_str();

                                    imports.insert(String::from(name), String::from(filepath));
                                }
                                _ => todo!("Not variable!"),
                            }
                        }
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
                                            variables.insert(
                                                String::from(name.as_str()),
                                                Variable {
                                                    var_type: VariableType::NUM,
                                                    value: 0isize,
                                                    var_index: variables.len(),
                                                },
                                            );
                                        }
                                        "list" => {
                                            variables.insert(
                                                String::from(name.as_str()),
                                                Variable {
                                                    var_type: VariableType::LIST,
                                                    value: -1isize,
                                                    var_index: variables.len(),
                                                },
                                            );
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

            let mut label_map: HashMap<String, String> = HashMap::new();
            let mut label_count = 0;

            let main = pre_main;

            match main.as_rule() {
                Rule::SubBlock => {
                    blocks.insert(String::from("main"), vec![]);
                    block_index.insert(String::from("main"), 0);

                    parse_subblocks(
                        String::from("main"),
                        main,
                        blocks,
                        block_index,
                        variables,
                        &mut label_map,
                        &mut label_count,
                        &imports,
                        true,
                    );
                }
                _ => panic!("MAIN not subblock?"),
            }

            let rule_blocks = v.next().unwrap();
            if rule_blocks.as_rule() != Rule::Blocks {
                panic!("NOT BLOCKS {:?}", v)
            }

            for b in rule_blocks.into_inner() {
                if b.as_rule() == Rule::EOI {
                    break;
                }

                if b.as_rule() != Rule::Block {
                    panic!("NOT BLOCKS {:?}", b)
                }

                parse_block(
                    b,
                    blocks,
                    block_index,
                    variables,
                    &mut label_map,
                    &mut label_count,
                    &imports,
                    true,
                );
            }

            blocks.insert(String::from("term"), vec![]); // TODO
            block_index.insert(String::from("term"), block_index.len());
        }
        _ => panic!(),
    }
}

impl SmplExecutor {
    // fn interpret_expr<I: std::io::Read, O: std::io::Write>(
    //     &mut self,
    //     e: Expr,
    //     input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
    //     output: &mut Option<O>,
    // ) -> bool {
    //     match e {
    //         Expr::Instr(cmd) => {
    //             cmd.interpret(&mut self.stack, input, output);
    //             false
    //         }
    //         Expr::Goto(l) => {
    //             self.label = l.get_label_name();
    //             true
    //         }
    //         Expr::Branch(thn, els) => {
    //             let a = self.stack.pop().unwrap();
    //             if a == 0 {
    //                 self.label = els.get_label_name();
    //             } else {
    //                 self.label = thn.get_label_name();
    //             }
    //             true
    //         }
    //         Expr::Debug => {
    //             println!("Debug {}: {:?}", self.label, self.stack);
    //             false
    //         }
    //         Expr::Comment(_) => false,
    //         Expr::Get(s) => {
    //             self.stack.push(self.variables[&s].value);
    //             false
    //         }
    //         Expr::Set(s) => {
    //             let v: Variable = self.variables[&s].clone();
    //             self.variables.insert(
    //                 s.clone(),
    //                 Variable {
    //                     value: self.stack.pop().unwrap(),
    //                     ..v
    //                 },
    //             );
    //             false
    //         }
    //     }
    // }

    // pub fn interpret<I: std::io::Read, O: std::io::Write>(
    //     &mut self,
    //     input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
    //     output: &mut Option<O>,
    // ) {
    //     while self.label != "term" {
    //         for expr in self.blocks[&self.label].clone() {
    //             if self.interpret_expr(expr, input, output) {
    //                 break;
    //             }
    //         }
    //     }
    // }

    pub fn new(filepath: &str, registers: usize) -> Self {
        let mut blocks = HashMap::new();
        let mut block_index = HashMap::new();

        let mut variables: HashMap<String, Variable> = HashMap::new();
        let mut imports: HashMap<String, String> = HashMap::new();

        // Add registers
        for _ in 0..registers {
            variables.insert(
                format!("__R{}__", variables.len()),
                Variable {
                    var_type: VariableType::NUM,
                    value: 0,
                    var_index: variables.len(),
                },
            );
        }

        parse_string(filepath, &mut blocks, &mut block_index, &mut variables, &mut imports);

        println!("variables {:?}", variables);
        SmplExecutor {
            blocks,
            block_index,
            variables,
            stack: Vec::new(),
            label: String::from("main"),
            registers,
            imports,
        }
    }

    // pub fn interpret_from_string<I: std::io::Read, O: std::io::Write>(
    //     unparsed: &str,
    //     input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
    //     output: &mut Option<O>,
    //     registers: usize,
    // ) {
    //     SmplExecutor::new(unparsed, registers).interpret(input, output);
    // }

    // pub fn run_on_string(mut self, input: &str) -> String {
    //     let str_inp: Box<dyn std::io::Read> = Box::new(input.as_bytes());
    //     let stk_input: std::iter::Peekable<std::io::Bytes<_>> = str_inp.bytes().peekable();

    //     let mut stk_byt_out = vec![];
    //     {
    //         let stk_output: Box<dyn std::io::Write> = Box::new(&mut stk_byt_out);
    //         self.interpret(&mut Some(stk_input), &mut Some(stk_output));
    //     }

    //     String::from_utf8(stk_byt_out).unwrap()
    // }
}
