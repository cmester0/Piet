pub mod advc_to_mid_smpl;

use crate::advc::advc_to_mid_smpl::AdvcToSmpl;
use crate::piet_interpreter::CMD::{self};
use itertools::Itertools;
use num::*;
use pest::iterators::Pair;
use pest::*;
use pest_derive::Parser;
use std::collections::HashMap;
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
    var_type: VariableType,
    value: BigInt,
    var_index: usize,
}

impl VariableType {
    fn initial_value(self) -> BigInt {
        match self {
            VariableType::NUM => 0.into(),
            VariableType::LIST => (-1).into(),
        }
    }

    fn initialize_var(self, var_index: usize) -> Variable {
        Variable {
            var_type: self.clone(),
            value: self.initial_value(),
            var_index,
        }
    }
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

    Call(Label, Label),
    Return,

    Eq,
    Append,
    PrintListC,
    PrintListN,
    PrintCListOfList,
    In,
    Malloc,
    DupAtDepth,

    GetElem,
    SetElem,
    GetHeap,
    SetHeap,
    Readlines,
    Length,
    Index(String, Vec<(String, BigInt)>),
    ClearList(String),

    Print(String),
    LocalVar(String, VariableType),

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
    let actual_label = format!("advc_l_ref_{}", label_count);
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
    loop_breaks: &Vec<(Label, Label, String)>,
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

        Rule::LocalVar => {
            let name = String::from(e.next().unwrap().as_str());
            let var_type = match e.next().unwrap().as_str() {
                "num" => VariableType::NUM,
                "list" => VariableType::LIST,
                _ => panic!("Incorrect var type for {}", name),
            };
            LocalVar(name, var_type)
        }
        Rule::Eq => Eq,
        Rule::Append => Append,
        Rule::PrintListC => PrintListC,
        Rule::PrintListN => PrintListN,
        Rule::PrintCListOfList => PrintCListOfList,
        Rule::Print => Print(String::from(e.next().unwrap().as_str())),
        Rule::In => In,
        Rule::SetHeap => SetHeap,
        Rule::GetHeap => GetHeap,
        Rule::Malloc => Malloc,
        Rule::DupAtDepth => DupAtDepth,
        Rule::GetElem => GetElem,
        Rule::SetElem => SetElem,
        Rule::Readlines => Readlines,
        Rule::Length => Length,
        Rule::Nop => Instr(CMD::Nop),
        Rule::Index => {
            let mut indx_stmt = ne.into_inner();
            let name = String::from(indx_stmt.next().unwrap().as_str());

            let mut index_vec = vec![];
            while let Some(nst) = indx_stmt.next() {
                let mut nos = nst.into_inner();
                let n = String::from(nos.next().unwrap().as_str());
                let v: BigInt = if let Some(v) = nos.next() {
                    (if v.as_rule() == Rule::Negative {
                        Into::<BigInt>::into(-1)
                    } else {
                        1.into()
                    }) * (nos.next().unwrap().as_str().parse::<isize>().unwrap())
                } else {
                    0.into()
                };

                index_vec.push((n, v));
            }
            Index(name, index_vec)
        }

        Rule::Continue => {
            let (start_label, _, loop_var) = loop_breaks.last().unwrap();

            // end of body
            blocks.get_mut(&block_name.clone()).unwrap().extend(vec![
                Expr::Get(loop_var.clone()),
                Expr::Instr(CMD::Push(1.into())),
                Expr::Instr(CMD::Add),
                Expr::Set(loop_var.clone()),
            ]);

            Goto(start_label.clone())
        }
        Rule::Break => {
            let mut break_stmt = ne.into_inner();

            let (_, done_label, _) = loop_breaks[loop_breaks.len()
                - 1
                - break_stmt
                    .next()
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap()]
            .clone();
            Goto(done_label)
        }
        Rule::For => {
            let mut for_stmt = ne.into_inner();
            let start = String::from(for_stmt.next().unwrap().as_str());
            let end = String::from(for_stmt.next().unwrap().as_str());

            let start_label = new_label(
                String::from("for_start"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            // Go to condition check
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(Expr::Goto(start_label.clone()));

            let body_label = new_label(
                String::from("for_body"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            let done_label = new_label(
                String::from("for_done"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            // Go to condition check
            blocks
                .get_mut(&start_label.clone().get_label_name())
                .unwrap()
                .extend(vec![
                    Expr::Get(start.clone()),
                    Expr::Get(end.clone()),
                    Expr::Eq,
                    Expr::Branch(done_label.clone(), body_label.clone()),
                ]);

            let mut sub_loop_breaks = loop_breaks.clone();
            sub_loop_breaks.push((start_label.clone(), done_label.clone(), start.clone()));
            block_name = parse_subblocks(
                body_label.clone().get_label_name(),
                for_stmt.next().unwrap(),
                blocks,
                block_index,
                variables,
                label_map,
                label_count,
                imports,
                &sub_loop_breaks,
            );

            // end of body
            blocks.get_mut(&block_name.clone()).unwrap().extend(vec![
                Expr::Get(start.clone()),
                Expr::Instr(CMD::Push(1.into())),
                Expr::Instr(CMD::Add),
                Expr::Set(start.clone()),
                Expr::Goto(start_label.clone()),
            ]);

            block_name = done_label.get_label_name();

            For(start, end, start_label)
        }
        Rule::ForStart => {
            let mut for_stmt = ne.into_inner();
            let start = String::from(for_stmt.next().unwrap().as_str());

            let n = for_stmt.next().unwrap();
            let initial: BigInt = match n.as_rule() {
                Rule::Number => n.as_str().parse().unwrap(),
                Rule::Char => (n.as_str().chars().next().unwrap() as isize).into(),
                _ => panic!("Trying to push non-number"),
            };
            let end = String::from(for_stmt.next().unwrap().as_str());

            let start_label = new_label(
                String::from("for_start"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            // Go to condition check
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(Expr::Instr(CMD::Push(initial)));
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(Expr::Set(start.clone()));
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(Expr::Goto(start_label.clone()));

            let body_label = new_label(
                String::from("for_body"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            let done_label = new_label(
                String::from("for_done"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            // Go to condition check
            blocks
                .get_mut(&start_label.clone().get_label_name())
                .unwrap()
                .extend(vec![
                    Expr::Get(start.clone()),
                    Expr::Get(end.clone()),
                    Expr::Eq,
                    Expr::Branch(done_label.clone(), body_label.clone()),
                ]);

            let mut sub_loop_breaks = loop_breaks.clone();
            sub_loop_breaks.push((start_label.clone(), done_label.clone(), start.clone()));
            block_name = parse_subblocks(
                body_label.clone().get_label_name(),
                for_stmt.next().unwrap(),
                blocks,
                block_index,
                variables,
                label_map,
                label_count,
                imports,
                &sub_loop_breaks,
            );

            // end of body
            blocks.get_mut(&block_name.clone()).unwrap().extend(vec![
                Expr::Get(start.clone()),
                Expr::Instr(CMD::Push(1.into())),
                Expr::Instr(CMD::Add),
                Expr::Set(start.clone()),
                Expr::Goto(start_label.clone()),
            ]);

            block_name = done_label.get_label_name();

            For(start, end, start_label)
        }
        Rule::If => {
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
                .push(Expr::Branch(if_label.clone(), else_label.clone()));

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
                loop_breaks,
            );

            // Go to condition check
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(Expr::Goto(continue_label.clone()));

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
                loop_breaks,
            );

            // Go to condition check
            blocks
                .get_mut(&block_name.clone())
                .unwrap()
                .push(Expr::Goto(continue_label.clone()));

            block_name = continue_label.clone().get_label_name();

            If(
                Label::Ref(if_label.clone().get_label_name()),
                Label::Ref(else_label.clone().get_label_name()),
            )
        }
        Rule::Call => {
            let mut call_stmt = ne.into_inner();
            let function_label = Label::parse_label(call_stmt.next().unwrap(), label_map);

            let return_label = new_label(
                String::from("return_label"),
                blocks,
                block_index,
                label_map,
                label_count,
            );

            block_name = return_label.clone().get_label_name();
            Call(function_label, return_label)
        }
        Rule::Return => Return,

        Rule::ClearList => {
            let mut clear_list_stmt = ne.into_inner();
            let list = String::from(clear_list_stmt.next().unwrap().as_str());
            ClearList(list)
        }

        x => panic!("unmatched expression {:?}", x),
    };

    (block_name, expr)
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    pub stack: Vec<BigInt>, // Stack frames
    pub label: Label,       // Return label
    pub variables: HashMap<String, Variable>,
}

#[derive(Clone)]
pub struct AdvcExecutor {
    pub blocks: HashMap<String, Vec<Expr>>,
    pub block_index: HashMap<String, usize>,
    pub variables: HashMap<String, Variable>,
    pub stack_frames: Vec<StackFrame>, // Stack frames
    pub heap: Vec<BigInt>,
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
    loop_breaks: &Vec<(Label, Label, String)>,
) -> String {
    if sub_block.as_rule() != Rule::SubBlock {
        panic!()
    }

    for x in sub_block.into_inner() {
        match x.as_rule() {
            Rule::NewLabel => {
                let ref_label = String::from(x.into_inner().next().unwrap().as_str());
                let actual_label = format!("advc_l_ref_{}", label_count);
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
                            loop_breaks,
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
                        loop_breaks,
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
                    loop_breaks,
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
    loop_breaks: &Vec<(Label, Label, String)>,
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
        loop_breaks,
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
        .expect(format!("unsuccessful parse of\n{}", filepath).as_str())
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
                        &vec![],
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
                    &vec![],
                );
            }

            blocks.insert(String::from("term"), vec![]); // TODO
            block_index.insert(String::from("term"), block_index.len());
        }
        _ => panic!(),
    }
}

impl AdvcExecutor {
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
                    value: 0.into(),
                    var_index: variables.len(),
                },
            );
        }

        parse_string(filepath, &mut blocks, &mut block_index, &mut variables);

        AdvcExecutor {
            blocks,
            block_index,
            variables,
            stack_frames: vec![StackFrame {
                stack: Vec::new(),
                label: Label::Name(String::from("term")),
                variables: HashMap::new(),
            }], // Default for "main"?
            heap: Vec::new(),
            label: String::from("main"),
            registers,
        }
    }

    pub fn get_variable(&mut self, s: &String) -> BigInt {
        if self.stack_frames.last().unwrap().variables.contains_key(s) {
            self.stack_frames.last().unwrap().variables[s]
                .value
                .clone()
        } else {
            if !self.variables.contains_key(s) {
                panic!("Get for variable {} does not exists", s);
            }
            self.variables[s].value.clone()
        }
    }

    pub fn set_variable(&mut self, s: &String, v: BigInt) {
        if self.stack_frames.last().unwrap().variables.contains_key(s) {
            self.stack_frames
                .last_mut()
                .unwrap()
                .variables
                .get_mut(s)
                .unwrap()
                .value = v;
        } else {
            if !self.variables.contains_key(s) {
                panic!("Set for variable {} does not exists", s);
            }
            self.variables.get_mut(s).unwrap().value = v;
        }
    }

    pub fn interpret_label<I: std::io::Read, O: std::io::Write>(
        &mut self,
        label: String,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) -> bool {
        for e in self.blocks[&label].clone() {
            match e {
                Instr(c) => {
                    c.interpret(
                        &mut self.stack_frames.last_mut().unwrap().stack,
                        input,
                        output,
                    );
                }
                Goto(l) => {
                    self.label = l.clone().get_label_name();
                    return self.label != "term";
                }
                Branch(l_then, l_else) => {
                    let a = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    if a == 0.into() {
                        self.label = l_else.clone().get_label_name();
                    } else {
                        self.label = l_then.clone().get_label_name();
                    }
                    return self.label != "term";
                }
                Debug => {
                    println!();
                    // println!("Heap: {:?}", self.heap);
                    println!(
                        "Variables: {:?}",
                        self.variables
                            .iter()
                            .sorted_by(|(_, v1), (_, v2)| v1.var_index.cmp(&v2.var_index))
                            .map(|(x, v)| (x, v.value.clone()))
                            .collect::<Vec<_>>()
                    );
                    println!(
                        "Local variables: {:?}",
                        self.stack_frames.last().unwrap().variables
                            .iter()
                            .sorted_by(|(_, v1), (_, v2)| v1.var_index.cmp(&v2.var_index))
                            .map(|(x, v)| (x, v.value.clone()))
                            .collect::<Vec<_>>()
                    );
                    println!(
                        "Stack_frame {}, stack: {:?}",
                        self.label,
                        self.stack_frames.last().unwrap().stack
                    );
                    println!();
                }
                Set(v) => {
                    let a = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    self.set_variable(&v, a);
                }
                Get(v) => {
                    let a = self.get_variable(&v);
                    self.stack_frames.last_mut().unwrap().stack.push(a);
                }
                For(_, _, l) => {
                    self.label = l.clone().get_label_name();
                    return self.label != "term";
                }
                If(l_then, l_else) => {
                    if self.stack_frames.last_mut().unwrap().stack.pop().unwrap() != 0.into() {
                        self.label = l_then.clone().get_label_name();
                    } else {
                        self.label = l_else.clone().get_label_name();
                    }
                    return self.label != "term";
                }
                Call(fun, ret) => {
                    self.stack_frames.push(StackFrame {
                        stack: Vec::new(),
                        label: ret,
                        variables: HashMap::new(),
                    });
                    // Goto function
                    self.label = fun.clone().get_label_name();
                    return self.label != "term";
                }
                Return => {
                    assert_eq!(self.stack_frames.last().unwrap().stack.len(), 0);
                    self.label = self.stack_frames.pop().unwrap().label.get_label_name();
                    return self.label != "term";
                }
                Eq => {
                    let a = self.stack_frames.last_mut().unwrap().stack.pop();
                    let b = self.stack_frames.last_mut().unwrap().stack.pop();
                    self.stack_frames.last_mut().unwrap().stack.push(if a == b {
                        1.into()
                    } else {
                        0.into()
                    });
                }
                Append => {
                    let ai = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    // Array doubling

                    let mut a = if ai == (-1).into() {
                        let na = self.heap.len();
                        self.heap.push(1.into());
                        self.heap.push(0.into());
                        self.heap.push(0.into());
                        na
                    } else {
                        ai.to_usize().unwrap()
                    };

                    if self.heap[a] == self.heap[a + 1] {
                        let na = self.heap.len();
                        self.heap.push(self.heap[a].clone() * 2);
                        self.heap.push(self.heap[a + 1].clone());
                        self.heap.extend(
                            self.heap[(a + 2)..(a + 2 + (self.heap[a].to_usize().unwrap()))]
                                .iter()
                                .cloned()
                                .collect::<Vec<_>>(),
                        );

                        for _ in 0..self.heap[a].to_usize().unwrap() {
                            self.heap.push(0.into())
                        }
                        a = na;
                    };

                    let index = a + 2 + self.heap[a + 1].to_usize().unwrap();
                    self.heap[index] = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    self.heap[a + 1] += 1;

                    self.stack_frames.last_mut().unwrap().stack.push(a.into());
                }
                PrintListC => {
                    let output = output.as_mut().unwrap();
                    write!(output, "[").unwrap();
                    let l = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let l_len = self.heap
                        [(l.clone() + Into::<BigInt>::into(1)).to_usize().unwrap()]
                    .clone();
                    for c in self.heap[(l.clone() + Into::<BigInt>::into(2)).to_usize().unwrap()
                        ..(l.clone() + Into::<BigInt>::into(2) + l_len)
                            .clone()
                            .to_usize()
                            .unwrap()]
                        .into_iter()
                        .cloned()
                    {
                        write!(output, "{},", c.to_u8().unwrap() as char).unwrap();
                    }
                    writeln!(output, "]").unwrap();
                    // todo!("print_c_list_of_list")
                }
                PrintListN => {
                    let output = output.as_mut().unwrap();
                    write!(output, "[").unwrap();
                    let l = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let l_len = self.heap
                        [(l.clone() + Into::<BigInt>::into(1)).to_usize().unwrap()]
                    .clone();
                    for c in self.heap[(l.clone() + Into::<BigInt>::into(2)).to_usize().unwrap()
                        ..(l.clone() + Into::<BigInt>::into(2) + l_len)
                            .clone()
                            .to_usize()
                            .unwrap()]
                        .into_iter()
                        .cloned()
                    {
                        write!(output, "{},", c).unwrap();
                    }
                    writeln!(output, "]").unwrap();
                }
                PrintCListOfList => {
                    let output = output.as_mut().unwrap();
                    let l = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let l_len = self.heap
                        [(l.clone() + Into::<BigInt>::into(1)).to_usize().unwrap()]
                    .clone();
                    for ll in self.heap[(l.clone() + Into::<BigInt>::into(2)).to_usize().unwrap()
                        ..(l.clone() + Into::<BigInt>::into(2) + l_len)
                            .clone()
                            .to_usize()
                            .unwrap()]
                        .into_iter()
                        .cloned()
                    {
                        let ll_len = self.heap
                            [(ll.clone() + Into::<BigInt>::into(1)).to_usize().unwrap()]
                        .clone();
                        for c in
                            self.heap[(ll.clone() + Into::<BigInt>::into(2)).to_usize().unwrap()
                                ..(ll.clone() + Into::<BigInt>::into(2) + ll_len)
                                    .clone()
                                    .to_usize()
                                    .unwrap()]
                                .into_iter()
                                .cloned()
                        {
                            write!(output, "{}", c.to_u8().unwrap() as char).unwrap();
                        }
                        writeln!(output).unwrap();
                    }
                    // todo!("print_c_list_of_list")
                }
                Print(s) => {
                    let output = output.as_mut().unwrap();
                    write!(output, "{}", s).unwrap();
                }
                In => {
                    let z = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let l = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let a = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    // a in l starting at z
                    let mut index: BigInt = (-1).into();

                    for i in z.to_isize().unwrap()
                        ..self.heap[(l.clone() + Into::<BigInt>::into(1)).to_usize().unwrap()]
                            .clone()
                            .to_isize()
                            .unwrap()
                    {
                        if a == self.heap[(l.clone() + Into::<BigInt>::into(2) + i)
                            .to_usize()
                            .unwrap()]
                        .clone()
                        {
                            index = i.into();
                            break;
                        }
                    }
                    self.stack_frames
                        .last_mut()
                        .unwrap()
                        .stack
                        .push(index.into());
                }
                Malloc => {
                    todo!("Malloc")
                }
                DupAtDepth => {
                    let a = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let len = self.stack_frames.last_mut().unwrap().stack.len();
                    let b = self
                        .stack_frames
                        .last_mut()
                        .unwrap()
                        .stack
                        .get(len - a.to_usize().unwrap())
                        .unwrap()
                        .clone();
                    self.stack_frames.last_mut().unwrap().stack.push(b.clone());
                }
                GetElem => {
                    let a = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let b = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    self.stack_frames.last_mut().unwrap().stack.push(
                        self.heap[(a + Into::<BigInt>::into(2) + b).to_usize().unwrap()].clone(),
                    );
                }
                SetElem => {
                    let a = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let b = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let c = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    self.heap[(a + Into::<BigInt>::into(2) + b).to_usize().unwrap()] = c;
                }
                GetHeap => {
                    let a = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    self.stack_frames
                        .last_mut()
                        .unwrap()
                        .stack
                        .push(self.heap[a.to_usize().unwrap()].clone());
                }
                SetHeap => {
                    let a = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    let b = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    self.heap[a.to_usize().unwrap()] = b;
                }
                LocalVar(n, t) => {
                    let var_index = self.stack_frames.last().unwrap().variables.len();
                    self.stack_frames
                        .last_mut()
                        .unwrap()
                        .variables
                        .insert(n, t.initialize_var(var_index));
                }
                Readlines => {
                    // Read lines into
                    let input = input.as_mut().unwrap();
                    let mut lines: Vec<BigInt> = Vec::new();
                    let mut line: Vec<BigInt> = Vec::new();

                    let add_line = &mut |heap: &mut Vec<BigInt>, line: Vec<BigInt>| {
                        let mut p = 1;
                        while p < line.len() {
                            p *= 2;
                        }
                        heap.push(p.into());
                        heap.push(line.len().into());
                        heap.extend(line.clone());
                        for _ in 0..p - line.len() {
                            heap.push(0.into())
                        }
                    };

                    while let Some(Ok(c)) = input.next() {
                        if c == 10 {
                            // 10 = \n
                            lines.push(self.heap.len().into());
                            add_line(&mut self.heap, line);
                            line = Vec::new();
                        } else {
                            line.push(c.into())
                        }
                    }

                    // lines.push(self.heap.len() as isize);
                    // add_line(&mut self.heap, line);

                    self.stack_frames
                        .last_mut()
                        .unwrap()
                        .stack
                        .push(self.heap.len().into());
                    add_line(&mut self.heap, lines);
                }
                Length => {
                    let a = self.stack_frames.last_mut().unwrap().stack.pop().unwrap();
                    if a == (-1).into() {
                        self.stack_frames.last_mut().unwrap().stack.push(0.into())
                    } else {
                        self.stack_frames
                            .last_mut()
                            .unwrap()
                            .stack
                            .push(self.heap[a.to_usize().unwrap() + 1].clone());
                    }
                }
                Index(s, v) => {
                    let mut curr = self.get_variable(&s).clone();
                    let mut debug_print = format!("{}", s);
                    for (name, offset) in v {
                        let index = self.get_variable(&name).clone() + offset.clone();
                        debug_print = format!("{}[{}(={})+{}]", debug_print, name, index, offset);
                        if self.heap.len()
                            <= (curr.clone() + Into::<BigInt>::into(2isize) + index.clone())
                                .to_usize()
                                .unwrap()
                        {
                            panic!("{}", debug_print);
                        }
                        curr = self.heap[(curr + Into::<BigInt>::into(2isize) + index)
                            .to_usize()
                            .unwrap()]
                        .clone();
                    }
                    self.stack_frames.last_mut().unwrap().stack.push(curr);
                }
                ClearList(l) => {
                    let index = self.get_variable(&l).clone();
                    self.heap[(index + Into::<BigInt>::into(1isize)).to_usize().unwrap()] =
                        0.into();
                }
                Comment(_) => {}
            }
        }

        true
    }

    pub fn interpret<I: std::io::Read, O: std::io::Write>(
        &mut self,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) {
        while self.interpret_label(self.label.clone(), input, output) {}
    }

    pub fn handle_advc(
        &mut self,
        run: bool,
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
        if run {
            let input = std::io::stdin().bytes().peekable();
            let output = std::io::stdout();

            self.interpret(&mut Some(input), &mut Some(output));
        }

        if !(output.is_some() || to_stk.is_some() || run_stk || to_piet.is_some() || run_piet) {
            return;
        }

        let smpl_executor = AdvcToSmpl::to_smpl(self.clone());
        smpl_executor.handle_smpl(
            output,
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
