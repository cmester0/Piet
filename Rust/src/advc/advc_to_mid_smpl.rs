use itertools::Itertools;

use super::AdvcExecutor;
use crate::advc::{
    Expr as AdvcExpr, Label as AdvcLabel, Variable as AdvcVariable,
    VariableType as AdvcVariableType,
};
use crate::mid_smpl::{
    Expr::{self, *},
    Label, SmplExecutor, Variable, VariableType,
};
use crate::piet_interpreter::CMD::{self, *};
use std::collections::{HashMap, HashSet};

pub struct AdvcToSmpl {
    advc_executor: AdvcExecutor,
    smpl_executor: SmplExecutor,
    local_vars: HashMap<String, HashMap<String, Variable>>,
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

        self.smpl_executor
            .blocks
            .get_mut(&self.smpl_executor.label)
            .unwrap()
            .push(Expr::Comment(format!("+lib_{}", lib)));

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

        self.smpl_executor
            .blocks
            .get_mut(&self.smpl_executor.label)
            .unwrap()
            .push(Expr::Comment(format!("-lib_{}", lib)));

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

    fn handle_advc_instr(&mut self, local_label: String, e: AdvcExpr) {
        match e {
            AdvcExpr::Instr(Nop) => {}
            AdvcExpr::Instr(c @ Push(_)) => {
                self.add_cmd(c.clone());
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
                if self.local_vars.contains_key(&local_label.clone())
                    && self
                        .local_vars
                        .get(&local_label)
                        .unwrap()
                        .contains_key(&var)
                {
                    let local_var_index = self.local_vars.get(&local_label.clone()).unwrap().get(&var).unwrap().var_index;

                    self.add_expr(Expr::Get(String::from("base_pointer")));

                    self.add_expr(Expr::Instr(Dup));
                    self.add_cmd(Push(1.into()));
                    self.add_expr(Expr::Instr(Add));

                    self.add_lib(String::from("swap_smpl"));
                    self.add_lib(String::from("sub"));

                    self.add_cmd(Push(local_var_index.into()));
                    self.add_lib(String::from("push"));
                    self.add_cmd(Push(1.into()));
                    self.add_lib(String::from("push"));
                    self.add_lib(String::from("add"));

                    self.add_lib(String::from("sub"));

                    self.add_lib(String::from("swap_at_depth_smpl"));

                    self.add_lib(String::from("pop"));

                } else {
                    self.add_expr(Expr::Set(var));
                }
            }
            AdvcExpr::Get(var) => {
                if self.local_vars.contains_key(&local_label.clone())
                    && self
                        .local_vars
                        .get(&local_label)
                        .unwrap()
                        .contains_key(&var)
                {
                    let local_var_index = self.local_vars.get(&local_label.clone()).unwrap().get(&var).unwrap().var_index;

                    self.add_expr(Expr::Get(String::from("base_pointer")));

                    self.add_expr(Expr::Instr(Dup));
                    self.add_cmd(Push(1.into()));
                    self.add_expr(Expr::Instr(Add));

                    self.add_lib(String::from("swap_smpl"));
                    self.add_lib(String::from("sub"));

                    self.add_cmd(Push(local_var_index.into()));
                    self.add_lib(String::from("push"));
                    self.add_cmd(Push(1.into()));
                    self.add_lib(String::from("push"));
                    self.add_lib(String::from("add"));
                    self.add_lib(String::from("sub"));

                    self.add_lib(String::from("dup_at_depth_smpl"));

                    // todo!("handle local variable {}", var);
                } else {
                    self.add_expr(Expr::Get(var));
                }
            }
            AdvcExpr::Eq => {
                self.add_lib(String::from("eq"));
            }
            AdvcExpr::Append => {
                self.add_lib(String::from("append"));
            }
            AdvcExpr::PrintListC => {
                self.add_lib(String::from("print_listC"));
            }
            AdvcExpr::PrintListN => {
                self.add_lib(String::from("print_listN"));
            }
            AdvcExpr::PrintCListOfList => {
                self.add_lib(String::from("printC_list_of_list"));
            }
            AdvcExpr::Print(s) => {
                for c in s.chars() {
                    self.add_expr(Instr(CMD::Push((c as isize).into())));
                    self.add_lib(String::from("push"));
                    self.add_lib(String::from("outC"));
                }
            }
            AdvcExpr::LocalVar(_, _) => {
                // Already handled!
            }
            AdvcExpr::In => {
                self.add_lib(String::from("in"));
            }
            AdvcExpr::Malloc => {
                self.add_lib(String::from("malloc"));
            }
            AdvcExpr::DupAtDepth => {
                self.add_lib(String::from("dup_at_depth_smpl"));
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
                let mut exprs = vec![AdvcExpr::Get(name)];
                for (n, v) in indexes {
                    exprs.push(AdvcExpr::Get(n));
                    if v == 0.into() {
                    } else if v < 0.into() {
                        exprs.push(AdvcExpr::Instr(Push(-v)));
                        exprs.push(AdvcExpr::Instr(Sub));
                    } else {
                        exprs.push(AdvcExpr::Instr(Push(v)));
                        exprs.push(AdvcExpr::Instr(Add));
                    }

                    exprs.push(AdvcExpr::GetElem);
                }

                self.handle_advc_instr(
                    local_label.clone(),
                    AdvcExpr::Comment(String::from("+index")),
                );
                for x in exprs {
                    self.handle_advc_instr(local_label.clone(), x);
                }
                self.handle_advc_instr(local_label, AdvcExpr::Comment(String::from("-index")));
            }
            AdvcExpr::For(_, _, _) => {
                // NOP
            }
            AdvcExpr::If(_, _) => {
                // NOP
            }
            AdvcExpr::Call(a, r) => {
                // Add return label address

                if !self
                    .smpl_executor
                    .block_index
                    .contains_key(&r.clone().get_label_name())
                {
                    panic!("smpl executor: {}", r.clone().get_label_name());
                }
                self.add_expr(Instr(CMD::Push(
                    self.smpl_executor.block_index[&r.clone().get_label_name()]
                        .clone()
                        .into(),
                )));
                self.add_lib(String::from("push"));

                // Set base pointer of call frame (to something consistent)
                self.add_expr(Get(String::from("base_pointer")));

                self.add_expr(Instr(CMD::Dup));
                self.add_expr(Instr(CMD::Push(1.into())));
                self.add_expr(Instr(CMD::Add));

                self.add_expr(Set(String::from("base_pointer")));

                // Push all the local variables
                let variable_map: HashMap<_, _> = self
                    .local_vars
                    .get(&a.clone().get_label_name())
                    .unwrap()
                    .clone();
                for (_var_name, var_def) in variable_map
                    .clone()
                    .into_iter()
                    .sorted_by(|(_, var1), (_, var2)| var1.var_index.cmp(&var2.var_index))
                {
                    self.add_expr(Instr(CMD::Push(var_def.value)));
                    self.add_lib(String::from("push"));
                }

                // self.add_expr(Instr(CMD::Push(1.into())));
                // self.add_lib(String::from("push"));

                self.add_expr(Goto(handle_label(a)));

                self.smpl_executor.label = r.get_label_name();

            }
            AdvcExpr::Return => {
                self.add_expr(Expr::Debug);

                let variable_map: HashMap<_, _> = self
                    .local_vars
                    .get(&local_label.clone())
                    .unwrap()
                    .clone();

                for _ in 0..variable_map.len() {
                    self.add_lib(String::from("pop"));
                }

                self.add_expr(Set(String::from("base_pointer")));

                self.add_expr(Expr::Debug);

                // TODO: assumes stack must be empty ..
                // self.add_lib(String::from("pop"));
                self.add_expr(Instr(CMD::Push(1.into())));
                self.add_expr(Instr(CMD::Sub));

                self.add_lib(String::from("swap"));

                self.add_expr(Expr::Debug);

                self.add_expr(GotoStk);
            }
            AdvcExpr::ClearList(l) => {
                self.add_expr(Instr(CMD::Push(0.into())));
                self.add_lib(String::from("push"));
                self.add_expr(Expr::Get(l));
                self.add_expr(Instr(CMD::Push(1.into())));
                self.add_lib(String::from("push"));
                self.add_lib(String::from("add"));
                self.add_lib(String::from("set_heap"));
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
            local_vars: HashMap::new(),
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
            "readC_until",
            "readlines",
            "roll",
            "set_at_depth",
            "set_elem",
            "set_heap",
            "stk_eq",
            "sub",
            "swap",
            "swap_at_depth",
            "swap_at_depth_smpl",
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
        advc_to_smpl.smpl_executor.variables.insert(
            String::from("base_pointer"),
            VariableType::NUM.initialize_var(advc_to_smpl.smpl_executor.variables.len()),
        );

        let mut bi = 0;

        // Frame

        // Setup main
        advc_to_smpl
            .smpl_executor
            .block_index
            .insert(String::from("main"), bi);
        bi += 1;

        // Add labels (Parse 1)
        for (x, _) in advc_to_smpl
            .advc_executor
            .block_index
            .clone()
            .into_iter()
            .collect_vec()
            .into_iter()
            .sorted_by(|(_, v1), (_, v2)| v1.cmp(v2))
        {
            if x.clone() != "main" {
                advc_to_smpl.smpl_executor.block_index.insert(x.clone(), bi);
                bi += 1;
            }
        }

        // Construct call graph

        let mut functions: HashSet<String> = HashSet::new();

        // What functions exists?
        for (x, _) in advc_to_smpl
            .advc_executor
            .block_index
            .clone()
            .into_iter()
            .collect_vec()
            .into_iter()
            .sorted_by(|(_, v1), (_, v2)| v1.cmp(v2))
        {
            let v = advc_to_smpl.advc_executor.blocks[&x.clone()].clone();
            for e in v.clone() {
                match e {
                    AdvcExpr::Call(f, r) => {
                        functions.insert(f.clone().get_label_name());
                    }
                    _ => (),
                }
            }
        }

        for f in functions {
            let mut stack: Vec<(usize, String, Vec<String>)> = vec![(0, f.clone(), vec![])];
            let mut local_variables: HashMap<String, Variable> = HashMap::new();
            let mut visited: HashSet<String> = HashSet::new();
            while stack.len() > 0 {
                let (level, l, r) = stack.pop().unwrap();

                if visited.contains(&l) {
                    continue;
                }
                visited.insert(l.clone());

                for expr in advc_to_smpl.advc_executor.blocks[&l].clone() {
                    match expr {
                        AdvcExpr::Goto(g) => stack.push((level + 1, g.get_label_name(), r.clone())),
                        AdvcExpr::Branch(t, e) => {
                            stack.push((level + 1, t.get_label_name(), r.clone()));
                            stack.push((level + 1, e.get_label_name(), r.clone()));
                        }
                        AdvcExpr::If(t, e) => {
                            stack.push((level + 1, t.get_label_name(), r.clone()));
                            stack.push((level + 1, e.get_label_name(), r.clone()));
                        }
                        AdvcExpr::For(_, _, l) => {
                            stack.push((level + 1, l.get_label_name(), r.clone()));
                        }
                        AdvcExpr::Return => {
                            continue;
                            // if r.len() > 0 {
                            //     let mut nr = r.clone();
                            //     let nf = nr.pop().unwrap();
                            //     stack.push((level + 1, nf, nr));
                            // }
                        }
                        AdvcExpr::Call(_cf, cr) => {
                            stack.push((level, cr.get_label_name(), r.clone()))
                            // let mut nr = r.clone();
                            // nr.push(cr.get_label_name());
                            // stack.push((level + 1, cf.get_label_name(), nr));
                        }
                        AdvcExpr::LocalVar(n, t) => {
                            local_variables.insert(
                                n,
                                handle_variable(t.initialize_var(local_variables.len())),
                            );
                        }
                        _ => (),
                    }
                }
            }

            for fl in visited {
                advc_to_smpl.local_vars.insert(fl, local_variables.clone());
            }
        }

        // // Walk call graph dfs:
        // for (f,r) in functions {
        //     println!("{} -> {}:", f, r);
        //     let mut stack : Vec<(usize,String)> = vec![(0,f)];
        //     let mut visited : HashSet<String> = HashSet::new();
        //     while stack.len() > 0 {
        //         let (level, l) = stack.pop().unwrap();
        //         println!("-{}{}", " ".repeat(level), l);

        //         if l == r {
        //             continue
        //         }

        //         if visited.contains(&l) {
        //             continue;
        //         }
        //         visited.insert(l.clone());

        //         for n in call_graph[&l].clone() {
        //             stack.push((level+1,n));
        //         }
        //     }
        // }
        // // println!("CALLGRAPH: {:?}", call_graph);
        // // println!("Functions: {:?}", functions);

        ////////////////////
        // Translate code //
        ////////////////////

        // Setup stack frame
        advc_to_smpl
            .smpl_executor
            .blocks
            .insert(String::from("main"), vec![]);
        advc_to_smpl.handle_advc_instr(
            String::from("main"),
            AdvcExpr::Instr(Push(advc_to_smpl.smpl_executor.block_index["term"].into())),
        );
        advc_to_smpl.handle_advc_instr(String::from("main"), AdvcExpr::Instr(Push(1.into())));

        // Add code (Parse 2)
        for (x, _) in advc_to_smpl
            .advc_executor
            .block_index
            .clone()
            .into_iter()
            .collect_vec()
            .into_iter()
            .sorted_by(|(_, v1), (_, v2)| v1.cmp(v2))
        {
            // if x.clone() == "term" {
            //     continue;
            // }

            let v = advc_to_smpl.advc_executor.blocks[&x.clone()].clone();

            if x.clone() != "main" {
                advc_to_smpl.smpl_executor.label = x.clone();
                advc_to_smpl.smpl_executor.blocks.insert(x.clone(), vec![]);
            }

            for e in v.clone() {
                // advc_to_smpl.add_expr(Comment(format!("+{:?}", e.clone())));
                advc_to_smpl.handle_advc_instr(x.clone(), e.clone());
                // advc_to_smpl.add_expr(Comment(format!("-{:?}", e)));
            }
        }

        advc_to_smpl.smpl_executor.label = String::from("main");
        advc_to_smpl.smpl_executor
    }
}
