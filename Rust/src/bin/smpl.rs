use clap::Parser as CliParser;

use pest::*;
use pest_derive::Parser;

use std::collections::HashMap;
use std::fs;
use std::io::Read;

use piet::piet_stack::expr::*;

#[derive(Parser)]
#[grammar = "bin/smpl.pest"] // relative to src
pub struct SmplParser;

#[derive(CliParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filepath: String,
    #[arg(short, long)]
    output: String,
}

// fn parse_expr(e: pest::iterators::Pair<Rule>) -> Expr
// {
//     if e.as_rule() != Rule::Expr { panic!() }
//     let mut e = e.into_inner(); // .next().unwrap();
//     match e.next().unwrap().as_rule() {
//         Rule::Push => Instr(CMD::Push(e.next().unwrap().as_str().parse().unwrap())),
//         Rule::Pop => Instr(CMD::Pop),
//         Rule::Not => Instr(CMD::Not),
//         Rule::Add => Instr(CMD::Add),
//         Rule::Greater => Instr(CMD::Greater),
//         Rule::Sub => Instr(CMD::Sub),
//         Rule::Div => Instr(CMD::Div),
//         Rule::Mod => Instr(CMD::Mod),
//         Rule::Mul => Instr(CMD::Mul),
//         Rule::Dup => Instr(CMD::Dup),
//         Rule::InN => Instr(CMD::InN),
//         Rule::InC => Instr(CMD::InC),
//         Rule::Goto => Goto(e.next().unwrap().as_str()),
//         Rule::Branch => Branch(e.next().unwrap().as_str(), e.next().unwrap().as_str()),
//         Rule::Debug => Debug,
//         Rule::OutC => Instr(CMD::OutC),
//         Rule::OutN => Instr(CMD::OutN),
//         Rule::Roll => Instr(CMD::Roll),
//         _ => panic!("unmatched expression"),
//     }
// }

fn pre_parse_file(
    name: &str,
    filepath: &str,
    import_definitions: &mut HashMap<&str, HashMap<&str, Expr>>,
    variables: &mut HashMap<&str, usize>,
) {
    let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");
    let document = SmplParser::parse(
        Rule::Document,
        unparsed_file.as_str())
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    match document.as_rule() {
        Rule::Document => {
            let mut it = document.into_inner();
            for import in it.next().unwrap().into_inner() {
                let mut it = import.into_inner();
                let name = it.next().unwrap().as_str();
                let filepath = it.next().unwrap().as_str();

                if !import_definitions.contains_key(name) {
                    pre_parse_file(name, filepath, import_definitions, variables);
                }
            }

            for variable in it.next().unwrap().into_inner() {
                let mut it = variable.into_inner();
                let name = it.next().unwrap().as_str();
                // variables.insert(name, variables.len());
            }

            // let vars = it.next().unwrap();
            // println!("{:#?}", document);
        }
        _ => panic!(),
    }
}

// fn parse_file(
//     name: &str,
//     filepath: &str,
//     import_definitions: &mut HashMap<&str, HashMap<&str, Expr>>,
//     variables: &mut HashMap<&str, usize>,
// ) {
//     let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");
//     let document = SmplParser::parse(Rule::Document, &unparsed_file)
//         .expect("unsuccessful parse")
//         .next()
//         .unwrap();

//     match document.as_rule() {
//         Rule::Document => {
//             let mut it = document.into_inner();
//             let imports = it.next().unwrap();
//             for i in imports.into_inner() {
//                 let mut it = i.into_inner();
//                 let name = it.next().unwrap().as_str();
//                 let filepath = it.next().unwrap().as_str();

//                 if !import_definitions.contains_key(name) {
//                     parse_file(name, filepath, import_definitions, variables);
//                 }
//             }

//             let variables = it.next().unwrap();
//             for v in variables.into_inner() {
//                 let mut it = v.into_inner();
//                 let name = it.next().unwrap().as_str();
//                 println!("variables")
//             }

//             // let vars = it.next().unwrap();
//             // println!("{:#?}", document);
//         }
//         _ => panic!(),
//     }
// }

fn main() {
    let args = Args::parse();

    let mut import_definitions: HashMap<&str, HashMap<&str, Expr>> = HashMap::new();
    let mut variables: HashMap<&str, usize> = HashMap::new();
    // pre_parse_file("", args.filepath.as_str(), &mut import_definitions, &mut variables);

    // let HashMap<&str, usize>;

    // let mut executor = PietStackExecutor { blocks: &blocks, stack: Vec::new(), label: "main" };
    // let mut input = std::io::stdin().bytes().peekable();
    // executor.interpret(&mut input);
}
