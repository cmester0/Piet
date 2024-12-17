use itertools::Itertools;

use super::AdvcExecutor;
use crate::advc::{
    Expr as AdvcExpr, Label as AdvcLabel, Variable as AdvcVariable,
    VariableType as AdvcVariableType,
};
use crate::mid_smpl::{
    expr::{
        Expr::{self, *},
        Label,
    },
    SmplExecutor, Variable, VariableType,
};
use crate::piet_interpreter::CMD::{self, *};
use std::collections::HashMap;

pub struct AdvcToSmpl {
    advc_executor: AdvcExecutor,
    smpl_executor: SmplExecutor,
}

fn handle_label(l: AdvcLabel) -> Label {
    match l {
        AdvcLabel::Name(s) => Label::Name(s),
        AdvcLabel::Ref(s) => Label::Ref(s),
    }
}

fn handle_variable(l: AdvcVariable) -> Variable {
    Variable {
        var_type: match l.var_type {
            AdvcVariableType::NUM => VariableType::NUM,
            AdvcVariableType::LIST => VariableType::LIST,
        },
        value: l.value,
        var_index: l.var_index,
    }
}

impl AdvcToSmpl {
    fn add_expr(&mut self, e: Expr) {
        self.smpl_executor
            .blocks
            .get_mut(&self.smpl_executor.label)
            .unwrap()
            .push(e);
    }

    fn add_cmd(&mut self, c: CMD) {
        self.add_expr(Instr(c));
    }

    #[allow(dead_code)]
    fn add_cmds(&mut self, c: Vec<CMD>) {
        self.smpl_executor
            .blocks
            .get_mut(&self.smpl_executor.label)
            .unwrap()
            .extend(c.into_iter().map(Instr).collect::<Vec<_>>());
    }

    fn add_lib(&mut self, lib: String) {
        // self.smpl_executor
        //     .imports
        //     .insert(lib.clone(), format!("./lib/{}.lib", lib));

        self.smpl_executor.label = crate::mid_smpl::handle_lib(
            self.smpl_executor.label.clone(),
            lib.clone(),
            &mut self.smpl_executor.blocks,
            &mut self.smpl_executor.block_index,
            &mut self.smpl_executor.variables,

            &mut self.smpl_executor.label_map,
            &mut self.smpl_executor.label_count,
            &mut self.smpl_executor.imports,
        );

        // // TODO:
        // self.smpl_executor
        //     .blocks
        //     .get_mut(&self.smpl_executor.label)
        //     .unwrap()
        //     .push(Expr::Lib(lib));
    }

    #[allow(dead_code)]
    fn new_label(&mut self) -> String {
        let ni = self.smpl_executor.block_index.len();
        let new_block_label = format!("l{}", ni);
        self.smpl_executor
            .blocks
            .insert(new_block_label.clone(), vec![]);
        self.smpl_executor
            .block_index
            .insert(new_block_label.clone(), ni);

        new_block_label
    }

    fn handle_advc_instr(&mut self, e: AdvcExpr) {
        match e {
            AdvcExpr::Instr(Nop) => {}
            AdvcExpr::Instr(c @ Push(_)) => {
                self.add_cmd(c);
                self.add_lib(c.cmd_str());
            }
            AdvcExpr::Instr(c) => {
                self.add_lib(c.cmd_str());
            }
            AdvcExpr::Goto(l) => {
                self.add_expr(Goto(handle_label(l)));
            }
            AdvcExpr::Branch(a, b) => {
                self.add_lib(String::from("pre_branch"));
                self.add_expr(Branch(handle_label(a), handle_label(b)));
            }
            AdvcExpr::Debug => {
                self.add_expr(Debug);
            }
            AdvcExpr::Comment(s) => {
                self.add_expr(Comment(s));
            }
            AdvcExpr::Set(var) => {
                self.add_expr(Expr::Set(var));
            }
            AdvcExpr::Get(var) => {
                self.add_expr(Expr::Get(var));
            }
            AdvcExpr::Eq => {
                self.add_lib(String::from("eq"));
            }
            AdvcExpr::Append => {
                self.add_lib(String::from("append"));
            }
            AdvcExpr::PrintCListOfList => {
                self.add_lib(String::from("printC_list_of_list"));
            }
            AdvcExpr::In => {
                self.add_lib(String::from("in"));
            }
            AdvcExpr::Malloc => {
                self.add_lib(String::from("malloc"));
            }
            AdvcExpr::GetElem => {
                self.add_lib(String::from("get_elem"));
            }
            AdvcExpr::SetElem => {
                self.add_lib(String::from("set_elem"));
            }
            AdvcExpr::GetHeap => {
                self.add_lib(String::from("get_heap"));
            }
            AdvcExpr::SetHeap => {
                self.add_lib(String::from("set_heap"));
            }
            AdvcExpr::Readlines => {
                self.add_lib(String::from("readlines"));
            }
            AdvcExpr::Length => {
                self.add_lib(String::from("length"));
            }
            AdvcExpr::Index(name, indexes) => {
                let mut exprs = vec![
                    AdvcExpr::Get(name),
                ];
                for (n, v) in indexes {
                    exprs.push(AdvcExpr::Get(n));
                    if v == 0 {
                    }
                    else if v < 0 {
                        exprs.push(AdvcExpr::Instr(Push(-v)));
                        exprs.push(AdvcExpr::Instr(Sub));
                    } else {
                        exprs.push(AdvcExpr::Instr(Push(v)));
                        exprs.push(AdvcExpr::Instr(Add));
                    }

                    exprs.push(AdvcExpr::GetElem);
                }

                self.handle_advc_instr(AdvcExpr::Comment(String::from("+index")));
                for x in exprs {
                    self.handle_advc_instr(x);
                }
                self.handle_advc_instr(AdvcExpr::Comment(String::from("-index")));
            }
            AdvcExpr::For(_, _, _) => {
                // NOP
            }
            AdvcExpr::If(_, _) => {
                // NOP
            }
        }
    }

    pub fn to_smpl(executor: AdvcExecutor) -> SmplExecutor {
        let mut advc_to_smpl = AdvcToSmpl {
            smpl_executor: SmplExecutor {
                blocks: HashMap::new(),
                block_index: HashMap::new(),
                variables: HashMap::new(),
                stack: vec![],
                label: String::from("main"),
                label_map: HashMap::new(),
                label_count: 0,
                registers: executor.registers.clone(),
                imports: HashMap::new(),
            },
            advc_executor: executor,
        };

        for s in vec![
            "add",
            "append",
            "copy_memory",
            "div",
            "dup",
            "dup_at_depth",
            "dup_at_depth_smpl",
            "eq",
            "get_at_depth",
            "get_elem",
            "get_heap",
            "get_list",
            "greater",
            "in",
            "inC",
            "inN",
            "length",
            "malloc",
            "mod",
            "mul",
            "not",
            "outC",
            "outN",
            "pop",
            "pre_branch",
            "printC_list_of_list",
            "print_listC",
            "print_listN",
            "push",
            "put_at_depth",
            "readC_until",
            "readlines",
            "roll",
            "set_elem",
            "set_heap",
            "stk_eq",
            "sub",
            "swap",
            "swap_at_depth",
            "swap_smpl",
        ] {
            advc_to_smpl
                .smpl_executor
                .imports
                .insert(String::from(s), String::from("stdlib"));
        }

        // Stack frame
        for (name, var) in advc_to_smpl.advc_executor.variables.clone()
        // .into_iter()
        // .sorted_by(|(_, a), (_, b)| a.var_index.cmp(&b.var_index).reverse())
        {
            advc_to_smpl
                .smpl_executor
                .variables
                .insert(name, handle_variable(var));
        }

        let mut bi = 0;

        // Setup stack invariants
        advc_to_smpl
            .smpl_executor
            .blocks
            .insert(String::from("main"), vec![]);
        advc_to_smpl
            .smpl_executor
            .block_index
            .insert(String::from("main"), bi);
        bi += 1;

        for (x, _) in advc_to_smpl
            .advc_executor
            .block_index
            .clone()
            .into_iter()
            .collect_vec()
            .into_iter()
            .sorted_by(|(_, v1), (_, v2)| v1.cmp(v2))
        {
            if x.clone() == "term" {
                continue;
            }

            let v = advc_to_smpl.advc_executor.blocks[&x.clone()].clone();

            if x.clone() != "main" {
                advc_to_smpl.smpl_executor.label = x.clone();
                advc_to_smpl.smpl_executor.blocks.insert(x.clone(), vec![]);
                advc_to_smpl.smpl_executor.block_index.insert(x.clone(), bi);
                bi += 1;
            }

            for e in v.clone() {
                // advc_to_smpl.add_expr(Comment(format!("+{:?}", e.clone())));
                advc_to_smpl.handle_advc_instr(e.clone());
                // advc_to_smpl.add_expr(Comment(format!("-{:?}", e)));
            }
        }

        advc_to_smpl.smpl_executor.label = String::from("main");
        advc_to_smpl.smpl_executor
    }
}
