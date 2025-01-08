pub mod mid_smpl_to_file;
pub mod mid_smpl_to_stk;

use crate::mid_smpl::mid_smpl_to_stk::SmplToStk;
use crate::piet_interpreter::CMD;
use num::*;
use pest::*;
use pest::iterators::Pair;
use pest_derive::Parser;
use phf::phf_map;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
#[allow(unused_imports)]
use std::io::Read;
use std::io::Write;

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
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

    Comment(String),

    Lib(String),
    GotoStk,
}
use Expr::*;

pub fn parse_expr(e: Pair<Rule>, label_map: &HashMap<String, String>) -> Expr {
    if e.as_rule() != Rule::Expr {
        panic!()
    }
    let mut e = e.into_inner(); // .next().unwrap();
    let ne = e.next().unwrap();
    match ne.as_rule() {
        Rule::Push => {
            let n = e.next().unwrap();
            match n.as_rule() {
                Rule::Number => Instr(CMD::Push(n.as_str().parse().unwrap())),
                Rule::Char => Instr(CMD::Push(
                    (n.as_str().chars().next().unwrap() as isize).into(),
                )),
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
        Rule::GotoStk => GotoStk,
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

        x => panic!("unmatched expression {:?}", x),
    }
}

#[derive(Clone, Debug)]
// #[repr(isize)]
pub enum VariableType {
    NUM = 0,
    LIST = -1,
}

impl VariableType {
    pub(crate) fn initial_value(self) -> BigInt {
        match self {
            VariableType::NUM => 0.into(),
            VariableType::LIST => (-1).into(),
        }
    }

    pub(crate) fn initialize_var(self, var_index: usize) -> Variable {
        Variable {
            var_type: self.clone(),
            value: self.initial_value(),
            var_index,
        }
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Variable {
    pub(crate) var_type: VariableType,
    pub(crate) value: BigInt,
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
    pub label_map: HashMap<String, String>,
    pub label_count: usize,
}

#[derive(Parser)]
#[grammar = "mid_smpl/mid_smpl.pest"] // relative to src
pub struct SmplParser;

static STDLIB: phf::Map<&str, &'static str> = phf_map! {
    "add" => include_str!("../../lib/add.lib"),
    "append" => include_str!("../../lib/append.lib"),
    "copy_memory" => include_str!("../../lib/copy_memory.lib"),
    "div" => include_str!("../../lib/div.lib"),
    "dup" => include_str!("../../lib/dup.lib"),
    "dup_at_depth" => include_str!("../../lib/dup_at_depth.lib"),
    "dup_at_depth_smpl" => include_str!("../../lib/dup_at_depth_smpl.lib"),
    "eq" => include_str!("../../lib/eq.lib"),
    "get_at_depth" => include_str!("../../lib/get_at_depth.lib"),
    "get_elem" => include_str!("../../lib/get_elem.lib"),
    "get_heap" => include_str!("../../lib/get_heap.lib"),
    "get_list" => include_str!("../../lib/get_list.lib"),
    "greater" => include_str!("../../lib/greater.lib"),
    "in" => include_str!("../../lib/in.lib"),
    "inC" => include_str!("../../lib/inC.lib"),
    "inN" => include_str!("../../lib/inN.lib"),
    "length" => include_str!("../../lib/length.lib"),
    "malloc" => include_str!("../../lib/malloc.lib"),
"mod" => include_str!("../../lib/mod.lib"),
    "mul" => include_str!("../../lib/mul.lib"),
    "not" => include_str!("../../lib/not.lib"),
    "outC" => include_str!("../../lib/outC.lib"),
    "outN" => include_str!("../../lib/outN.lib"),
    "pop" => include_str!("../../lib/pop.lib"),
    "pre_branch" => include_str!("../../lib/pre_branch.lib"),
    "printC_list_of_list" => include_str!("../../lib/printC_list_of_list.lib"),
    "print_listC" => include_str!("../../lib/print_listC.lib"),
    "print_listN" => include_str!("../../lib/print_listN.lib"),
    "push" => include_str!("../../lib/push.lib"),
    "put_at_depth" => include_str!("../../lib/put_at_depth.lib"),
    "readC_until" => include_str!("../../lib/readC_until.lib"),
    "readlines" => include_str!("../../lib/readlines.lib"),
    "roll" => include_str!("../../lib/roll.lib"),
    "set_elem" => include_str!("../../lib/set_elem.lib"),
    "set_heap" => include_str!("../../lib/set_heap.lib"),
    "stk_eq" => include_str!("../../lib/stk_eq.lib"),
    "sub" => include_str!("../../lib/sub.lib"),
    "swap" => include_str!("../../lib/swap.lib"),
    "swap_at_depth" => include_str!("../../lib/swap_at_depth.lib"),
    "swap_smpl" => include_str!("../../lib/swap_smpl.lib"),
};

pub fn handle_lib(
    mut block_name: String,
    lib_name: String,
    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    variables: &mut HashMap<String, Variable>,

    label_map: &mut HashMap<String, String>,
    label_count: &mut usize,
    imports: &HashMap<String, String>,
) -> String {
    let unparsed_file = if STDLIB.contains_key(&lib_name) {
        String::from(STDLIB[&lib_name])
    } else {
        if !imports.contains_key(&lib_name) {
            panic!("could not find key {}", lib_name);
        }
        let s = imports[&lib_name].clone();
        let actual_filepath = s;

        fs::read_to_string(actual_filepath.clone())
            .expect(format!("cannot read file {}", actual_filepath).as_str())
    };

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

    block_name
}

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

                if top_level {
                    blocks
                        .get_mut(&block_name)
                        .unwrap()
                        .push(Expr::Comment(format!("+{}", lib_name)));
                }

                block_name = handle_lib(
                    block_name.clone(),
                    lib_name.clone(),
                    blocks,
                    block_index,
                    variables,
                    label_map,
                    label_count,
                    imports,
                );

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
                                                    value: 0isize.into(),
                                                    var_index: variables.len(),
                                                },
                                            );
                                        }
                                        "list" => {
                                            variables.insert(
                                                String::from(name.as_str()),
                                                Variable {
                                                    var_type: VariableType::LIST,
                                                    value: (-1isize).into(),
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

        let label_map: HashMap<String, String> = HashMap::new();
        let label_count = 0;

        // Add registers
        for _ in 0..registers {
            variables.insert(
                format!("__R{}__", variables.len()),
                Variable {
                    var_type: VariableType::NUM,
                    value: 0.into(),
                    var_index: variables.len(),
                },
            );
        }

        parse_string(
            filepath,
            &mut blocks,
            &mut block_index,
            &mut variables,
            &mut imports,
        );

        SmplExecutor {
            blocks,
            block_index,
            variables,
            stack: Vec::new(),
            label: String::from("main"),
            label_map,
            label_count,
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

    pub fn handle_smpl(
        self,
        output: Option<String>,
        to_stk: Option<String>,
        optimize_stk: bool,
        run_stk: bool,
        to_piet: Option<String>,
        run_piet: bool,
        gui_piet: bool,
        steps_per_frame: usize,
        start_frame: usize,
        skip_whitespace: bool,
    ) {
        if output.is_some() {
            let file_str = self.to_file_string();
            let mut output_file = File::create(output.clone().unwrap()).unwrap();
            output_file.write(file_str.as_str().as_bytes()).unwrap();
        }

        if !(to_stk.is_some() || to_piet.is_some() || run_stk || run_piet) {
            return;
        }

        let stk_executor = SmplToStk::to_stk(self);
        stk_executor.handle_stk(
            to_stk,
            optimize_stk,
            run_stk,
            to_piet,
            run_piet,
            gui_piet,
            steps_per_frame,
            start_frame,
            skip_whitespace,
        );
    }
}
