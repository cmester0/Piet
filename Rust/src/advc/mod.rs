// mod expr;
pub mod advc_to_mid_smpl;

// use expr::*;
use pest::*;
use pest_derive::Parser;
use std::collections::HashMap;

use crate::piet_interpreter::CMD;

use pest::iterators::Pair;
use std::fs;

#[allow(unused_imports)]
use std::io::Read;

// use pest::iterators::Pair;
// use crate::piet_interpreter::CMD;

// use std::collections::HashMap;

#[derive(Clone, Debug)]
// #[repr(isize)]
pub enum VariableType {
    NUM = 0,
    LIST = -1,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Variable {
    var_type: VariableType,
    value: isize,
    var_index: usize,
}

#[derive(Debug, Clone)]
pub enum Label {
    Name(String),
    Ref(String),
}

impl Label {
    pub fn parse_label(e: Pair<Rule>, label_map: &HashMap<String, String>) -> Label {
        if e.as_rule() != Rule::Label {
            panic!("NOT LABEL {:?}\n{:?}", e.as_rule(), e)
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

    Set(String),
    Get(String),

    For(String, String, Label),
    If(Label, Label),

    Eq,

    Comment(String),
}
use Expr::*;

pub fn new_label(
    ref_label: String,
    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    label_map: &mut HashMap<String, String>,
    label_count: &mut usize,
) -> Label {
    let actual_label = format!("l_ref_{}", label_count);
    *label_count += 1;
    label_map.insert(ref_label, actual_label.clone());
    blocks.insert(actual_label.clone(), vec![Expr::Instr(CMD::Nop)]);
    block_index.insert(actual_label.clone(), block_index.len());
    Label::Ref(actual_label)
}

pub fn parse_expr(
    mut block_name: String,

    e: Pair<Rule>,

    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    variables: &mut HashMap<String, Variable>,

    label_map: &mut HashMap<String, String>,
    label_count: &mut usize,
    imports: &HashMap<String, String>,
) -> (String, Expr) {
    if e.as_rule() != Rule::Expr {
        panic!()
    }
    let mut e = e.into_inner(); // .next().unwrap();
    let ne = e.next().unwrap();
    let expr = match ne.as_rule() {
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

        Rule::Set => Set(String::from(e.next().unwrap().as_str())),
        Rule::Get => Get(String::from(e.next().unwrap().as_str())),

        Rule::For => {
            let mut for_stmt = ne.into_inner();
            let start = String::from(for_stmt.next().unwrap().as_str());
            let end = String::from(for_stmt.next().unwrap().as_str());

            let start_label = new_label(
                String::from("for"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            // Go to condition check
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(
                    Expr::Goto(start_label.clone())
                );

            let body_label = new_label(
                String::from("body"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            let done_label = new_label(
                String::from("done"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            // Go to condition check
            blocks
                .get_mut(&start_label.clone().get_label_name())
                .unwrap()
                .extend(
                    vec![
                        Expr::Get(start.clone()),
                        Expr::Get(end.clone()),
                        Expr::Eq,
                        Expr::Branch(done_label.clone(), body_label.clone()),
                    ]
                );

            block_name = parse_subblocks(
                body_label.clone().get_label_name(),
                for_stmt.next().unwrap(),
                blocks,
                block_index,
                variables,
                label_map,
                label_count,
                imports,
            );

            // end of body
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .extend(
                    vec![
                        Expr::Get(start.clone()),
                        Expr::Instr(CMD::Push(1)),
                        Expr::Instr(CMD::Add),
                        Expr::Set(start.clone()),
                        Expr::Goto(start_label.clone())
                    ]
                );

            block_name = done_label.get_label_name();

            For(start, end, start_label)
        }
        Rule::If => {
            println!("{}", ne.as_str());

            let mut if_stmt = ne.into_inner();

            let if_label = new_label(
                String::from("if"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            let else_label = new_label(
                String::from("else"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            let continue_label = new_label(
                String::from("continue"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            // Go to condition check
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(
                    Expr::Branch(if_label.clone(), else_label.clone())
                );

            // If:
            block_name = parse_subblocks(
                if_label.clone().get_label_name(),
                if_stmt.next().unwrap(),
                blocks,
                block_index,
                variables,
                label_map,
                label_count,
                imports,
            );

            // Go to condition check
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(
                    Expr::Goto(continue_label.clone())
                );

            // Else:
            block_name = parse_subblocks(
                else_label.clone().get_label_name(),
                if_stmt.next().unwrap(),
                blocks,
                block_index,
                variables,
                label_map,
                label_count,
                imports,
            );

            // Go to condition check
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(
                    Expr::Goto(continue_label.clone())
                );

            block_name = continue_label.clone().get_label_name();

            If(
                Label::Ref(if_label.clone().get_label_name()),
                Label::Ref(else_label.clone().get_label_name()),
            )
        }

        x => panic!("unmatched expression {:?}", x),
    };

    (block_name, expr)
}

#[derive(Clone)]
pub struct AdvcExecutor {
    pub blocks: HashMap<String, Vec<Expr>>,
    pub block_index: HashMap<String, usize>,
    pub variables: HashMap<String, Variable>,
    pub stack: Vec<isize>,
    pub heap: Vec<isize>,
    pub label: String,
    pub registers: usize,
}

#[derive(Parser)]
#[grammar = "advc/advc.pest"] // relative to src
pub struct AdvcParser;

pub fn parse_subblocks(
    mut block_name: String,
    sub_block: Pair<Rule>,
    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    variables: &mut HashMap<String, Variable>,

    label_map: &mut HashMap<String, String>,
    label_count: &mut usize,
    imports: &HashMap<String, String>,
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

                let unparsed_file = fs::read_to_string(actual_filepath.clone())
                    .expect(format!("cannot read file {}", actual_filepath).as_str());

                let mut v = AdvcParser::parse(Rule::LibBlocks, &unparsed_file)
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
                    );
                }
            }
            Rule::Expr => {
                let curr_name = block_name.clone();
                let (name, expr) = parse_expr(
                    block_name.clone(),
                    x,
                    blocks,
                    block_index,
                    variables,
                    label_map,
                    label_count,
                    imports,
                );
                block_name = name;
                blocks.get_mut(&curr_name).unwrap().push(expr);
            }
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
    )
}

pub fn parse_string(
    filepath: &str,
    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    variables: &mut HashMap<String, Variable>,
) {
    let unparsed =
        fs::read_to_string(filepath).expect(format!("cannot read file: {}", filepath).as_str());
    let document = AdvcParser::parse(Rule::Document, unparsed.as_str())
        .expect(format!("unsuccessful parse of {}", filepath).as_str())
        .next()
        .unwrap();

    let mut imports: HashMap<String, String> = HashMap::new();

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
                );
            }

            blocks.insert(String::from("term"), vec![]); // TODO
            block_index.insert(String::from("term"), block_index.len());
        }
        _ => panic!(),
    }
}

impl AdvcExecutor {
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

        parse_string(filepath, &mut blocks, &mut block_index, &mut variables);
        println!("variables {:?}", variables);
        AdvcExecutor {
            blocks,
            block_index,
            variables,
            stack: Vec::new(),
            heap: Vec::new(),
            label: String::from("main"),
            registers,
        }
    }

    // pub fn interpret_from_string<I: std::io::Read, O: std::io::Write>(
    //     unparsed: &str,
    //     input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
    //     output: &mut Option<O>,
    //     registers: usize,
    // ) {
    //     AdvcExecutor::new(unparsed, registers).interpret(input, output);
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
