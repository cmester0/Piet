mod expr;
pub mod smpl_to_stk;

use expr::*;
use pest::*;
use pest_derive::Parser;
use std::collections::HashMap;

use crate::piet_interpreter::CMD;

use pest::iterators::Pair;
use std::fs;

use std::path::*;
use std::io::Read;

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

#[derive(Clone)]
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

pub fn parse_subblocks(
    filepath: &str,
    mut block_name : String,
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

                let s = imports[&lib_name].clone();
                let actual_filepath = String::from(
                    Path::new(filepath)
                        .parent()
                        .unwrap()
                        .join(s.clone())
                        .to_str()
                        .unwrap(),
                );

                let unparsed_file =
                    fs::read_to_string(actual_filepath.clone()).expect("cannot read file");
                // parse

                let mut v = SmplParser::parse(Rule::LibBlocks, &unparsed_file)
                    .expect("unsuccessful parse")
                    .next()
                    .unwrap()
                    .into_inner();

                let initial_sub_block = v.next().unwrap();

                match initial_sub_block.as_rule() {
                    Rule::SubBlock => {
                        block_name = parse_subblocks(
                            actual_filepath.as_str(),
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

                    parse_block(
                        actual_filepath.as_str(),
                        b,
                        blocks,
                        block_index,
                        variables,
                        label_map,
                        label_count,
                        imports,
                    );
                }

                // HashMap<String, Vec<Expr>>, HashMap<String, usize>
                // parse_string(s);
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
    filepath: &str,
    b: Pair<Rule>,
    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    variables: &mut HashMap<String, Variable>,

    label_map: &mut HashMap<String, String>,
    label_count: &mut usize,
    imports: &HashMap<String, String>,
) {
    if b.as_rule() != Rule::Block {
        panic!();
    }

    let mut block = b.into_inner();
    if block.size_hint().0 == 0 {
        return;
    }

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
        filepath, name, sub_block,
        blocks, block_index, variables, label_map, label_count, imports
    );
}

pub fn parse_string(
    filepath: &str,
    blocks: &mut HashMap<String, Vec<Expr>>,
    block_index: &mut HashMap<String, usize>,
    variables: &mut HashMap<String, Variable>,
) {
    let unparsed = fs::read_to_string(filepath).expect("cannot read file");
    let document = SmplParser::parse(Rule::Document, unparsed.as_str())
        .expect("unsuccessful parse")
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
                                            variables.insert(
                                                String::from(name.as_str()),
                                                Variable::NUM(0isize),
                                            );
                                        }
                                        "list" => {
                                            variables.insert(
                                                String::from(name.as_str()),
                                                Variable::LIST(Vec::new()),
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
                    blocks.insert(
                        String::from("main"),
                        vec![]);
                    block_index.insert(String::from("main"), 0);

                    parse_subblocks(
                        filepath,
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
                    filepath,
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
            Expr::Comment(_) => false,
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

    pub fn new(filepath: &str) -> Self {
        let mut blocks = HashMap::new();
        let mut block_index = HashMap::new();
        let mut variables = HashMap::new();
        parse_string(filepath, &mut blocks, &mut block_index, &mut variables);
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

    pub fn run_on_string(mut self, input: &str) -> String{
        let str_inp: Box<dyn std::io::Read> = Box::new(input.as_bytes());
        let stk_input: std::iter::Peekable<std::io::Bytes<_>> = str_inp.bytes().peekable();

        let mut stk_byt_out = vec![];
        {
            let stk_output: Box<dyn std::io::Write> = Box::new(&mut stk_byt_out);
            self.interpret(
                &mut Some(stk_input),
                &mut Some(stk_output),
            );
        }

        String::from_utf8(stk_byt_out).unwrap()
    }
}
