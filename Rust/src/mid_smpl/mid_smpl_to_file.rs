use super::{Expr::*, VariableType};
use crate::piet_interpreter::CMD::*;
use itertools::Itertools;
use std::io::Write;

impl super::SmplExecutor {
    pub fn to_file_string(&self) -> String {
        let mut smpl_byt_out = vec![];
        {
            let mut smpl_output: Box<dyn std::io::Write> = Box::new(&mut smpl_byt_out);

            for (name, path) in self.imports.clone()
            // .into_iter()
            // .sorted_by(|(_, var1), (_, var2)| var1.var_index.cmp(&var2.var_index))
            {
                writeln!(smpl_output, "use {} {}", name, path).unwrap();
            }
            writeln!(smpl_output, "").unwrap();

            for (name, var) in self
                .variables
                .clone()
                .into_iter()
                .sorted_by(|(_, var1), (_, var2)| var1.var_index.cmp(&var2.var_index))
            {
                if var.var_index < self.registers {
                    continue;
                }

                writeln!(
                    smpl_output,
                    "var {} {}",
                    name,
                    match var.var_type {
                        VariableType::NUM => "num",
                        VariableType::LIST => "list",
                    }
                ).unwrap();
            }
            writeln!(smpl_output, "").unwrap();

            for (k, _) in self
                .block_index
                .clone()
                .into_iter()
                .sorted_by(|(_, v1), (_, v2)| v1.cmp(v2))
            {
                // Skip term block
                if k.clone() == "term" {
                    continue;
                }

                if k.clone() != "main" {
                    writeln!(smpl_output, "label {}", k.clone()).unwrap();
                }

                for e in self.blocks[&k].clone() {
                    match e {
                        Instr(c) => match c {
                            Nop => {}

                            Push(i) => writeln!(smpl_output, "push {}", i).unwrap(),
                            Pop => writeln!(smpl_output, "pop").unwrap(),
                            Add => writeln!(smpl_output, "add").unwrap(),
                            Sub => writeln!(smpl_output, "sub").unwrap(),
                            Mul => writeln!(smpl_output, "mul").unwrap(),
                            Div => writeln!(smpl_output, "div").unwrap(),
                            Mod => writeln!(smpl_output, "mod").unwrap(),
                            Not => writeln!(smpl_output, "not").unwrap(),
                            Greater => writeln!(smpl_output, "greater").unwrap(),
                            Dup => writeln!(smpl_output, "dup").unwrap(),
                            Roll => writeln!(smpl_output, "roll").unwrap(),
                            InN => writeln!(smpl_output, "inN").unwrap(),
                            InC => writeln!(smpl_output, "inC").unwrap(),
                            OutN => writeln!(smpl_output, "outN").unwrap(),
                            OutC => writeln!(smpl_output, "outC").unwrap(),

                            Pointer | Switch => panic!(),
                        },
                        Goto(s) => writeln!(smpl_output, "goto {}", s.get_label_name()).unwrap(),
                        Branch(a, b) => writeln!(
                            smpl_output,
                            "branch {} {}",
                            a.get_label_name(),
                            b.get_label_name()
                        )
                        .unwrap(),
                        Debug => writeln!(smpl_output, "debug").unwrap(),
                        Comment(s) => writeln!(smpl_output, "# {}", s).unwrap(),

                        Set(x) => writeln!(smpl_output, "set {}", x).unwrap(),
                        Get(x) => writeln!(smpl_output, "get {}", x).unwrap(),
                        Lib(x) => writeln!(smpl_output, "lib_{}", x).unwrap(),
                    }
                }
                writeln!(smpl_output, "").unwrap();
            }
        }
        String::from_utf8(smpl_byt_out).unwrap()
    }
}
