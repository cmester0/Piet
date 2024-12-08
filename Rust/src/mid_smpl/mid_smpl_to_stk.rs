use itertools::Itertools;

use super::Expr::{self as SmplExpr};
use super::SmplExecutor;
use crate::mid_smpl::*;
use crate::piet_stack::expr::Expr::{self, *};
use crate::{
    piet_interpreter::CMD::{self, *},
    piet_stack::PietStackExecutor,
};
use std::collections::HashMap;

pub struct SmplToStk {
    smpl_executor: SmplExecutor,
    stk_executor: PietStackExecutor,
}

impl SmplToStk {
    fn add_expr(&mut self, e: Expr) {
        self.stk_executor
            .blocks
            .get_mut(&self.stk_executor.label)
            .unwrap()
            .push(e);
    }

    fn add_cmd(&mut self, c: CMD) {
        self.add_expr(Instr(c));
    }

    fn add_cmds(&mut self, c: Vec<CMD>) {
        self.stk_executor
            .blocks
            .get_mut(&self.stk_executor.label)
            .unwrap()
            .extend(c.into_iter().map(Instr).collect::<Vec<_>>());
    }

    fn swap(&mut self) {
        self.add_cmds(vec![Push(2), Push(1), Roll]);
    }

    fn new_label(&mut self) -> String {
        let ni = self.stk_executor.block_index.len();
        let new_block_label = format!("l{}", ni);
        self.stk_executor
            .blocks
            .insert(new_block_label.clone(), vec![]);
        self.stk_executor
            .block_index
            .insert(new_block_label.clone(), ni);

        new_block_label
    }

    fn goto_new_label(&mut self) {
        let new_block_label = self.new_label();
        self.add_expr(Goto(new_block_label.clone()));
        self.stk_executor.label = new_block_label;
    }

    fn branch_new_labels(&mut self) -> (String, String) {
        let label1 = self.new_label();
        let label2 = self.new_label();

        self.add_expr(Branch(label1.clone(), label2.clone()));

        (label1, label2)
    }

    fn c_if(
        &mut self,
        success: impl Fn(&mut SmplToStk) -> (),
        fails: impl Fn(&mut SmplToStk) -> (),
    ) {
        let (succ_label, fail_label) = self.branch_new_labels();
        let continue_label = self.new_label();

        self.stk_executor.label = succ_label;
        success(self);
        self.add_expr(Goto(continue_label.clone()));

        self.stk_executor.label = fail_label;
        fails(self);
        self.add_expr(Goto(continue_label.clone()));

        self.stk_executor.label = continue_label;
    }

    fn dup_at_depth(&mut self) {
        // Save / update depth
        self.add_cmd(Dup);
        self.add_cmd(Push(1));
        self.add_cmd(Add);

        // Fetch the element
        self.add_cmd(Push(-1));
        self.add_cmd(Roll);

        // dup and save element
        self.add_cmd(Dup);
        self.add_cmd(Push(3));
        self.add_cmd(Push(-1));
        self.add_cmd(Roll);

        // Put back the new element
        self.add_cmd(Push(1));
        self.add_cmd(Add);
        self.add_cmd(Push(1));
        self.add_cmd(Roll);
    }

    fn dup_value_x_deep(&mut self, x: isize) {
        // Get the value to the top
        self.add_cmd(Push(x));
        self.add_cmd(Push(-1));
        self.add_cmd(Roll);

        self.add_cmd(Dup);

        // put it back
        self.add_cmd(Push(x + 1));
        self.add_cmd(Push(1));
        self.add_cmd(Roll);
    }

    fn swap_at_depth(&mut self) {
        // Save / update depth
        self.add_cmd(Dup);
        self.add_cmd(Push(1));
        self.add_cmd(Add);

        // Fetch the element
        self.add_cmd(Push(-1));
        self.add_cmd(Roll);

        // Do the swap
        self.add_cmd(Push(3));
        self.add_cmd(Push(1));
        self.add_cmd(Roll);

        // Put back the new element
        self.add_cmd(Push(1));
        self.add_cmd(Roll);
    }

    fn add_var(&mut self, var: Variable) {
        // Set variable index
        // (var_index.len() - var_index[i]) is actual index
        // self.smpl_executor.variables.insert(var_name, var.var_index);

        // Allocate empty variable
        self.add_cmd(Push(var.value));

        //////////////////////
        // Fetch stack size //
        //////////////////////

        self.swap();

        self.add_cmd(Push(1));
        self.add_cmd(Add);

        self.add_cmd(Dup);

        self.add_cmd(Push(3));
        self.add_cmd(Push(1));
        self.add_cmd(Roll);

        ////////////////////////
        // Rotate into bottom //
        ////////////////////////

        self.add_cmd(Push(1));
        self.add_cmd(Roll);
    }

    fn handle_smpl_instr(&mut self, e: SmplExpr) {
        match e {
            SmplExpr::Instr(c) => {
                self.add_cmd(c);
            }
            SmplExpr::Goto(l) => {
                self.add_expr(Goto(l.get_label_name()));
            }
            SmplExpr::Branch(a, b) => {
                self.add_expr(Branch(a.get_label_name(), b.get_label_name()));
            }
            SmplExpr::Debug => {
                self.add_expr(Debug);
            }
            SmplExpr::Comment(s) => {
                self.add_expr(Comment(s));
            }
            SmplExpr::Set(var) => {
                let var_index =
                // self.smpl_executor.variables.len() - 1 -
                    self.smpl_executor.variables[&var].clone().var_index;

                self.add_expr(Expr::Comment(format!("-{:?}", var)));

                self.add_cmd(Push((var_index + 1) as isize));

                self.swap();
                self.add_cmd(Dup);

                self.add_cmd(Push(3));
                self.add_cmd(Push(1));
                self.add_cmd(Roll);
                self.swap();
                self.add_cmd(Sub);

                self.add_cmd(Push(1));
                self.add_cmd(Add);

                self.add_cmd(Push(3));
                self.add_cmd(Push(-1));
                self.add_cmd(Roll);

                self.swap();

                self.swap_at_depth();
                self.add_cmd(Pop);
                self.add_cmd(Push(1));
                self.add_cmd(Sub);

                self.add_expr(Expr::Comment(format!("-{:?}", var)));
            }
            SmplExpr::Get(var) => {
                let var_index =
                    // self.smpl_executor.variables.len() - 1 -
                    self.smpl_executor.variables[&var].clone().var_index;

                self.add_expr(Expr::Comment(format!("+{:?}", var)));

                self.add_cmd(Push((var_index + 1) as isize));
                self.swap();
                self.add_cmd(Dup);

                self.add_cmd(Push(3));
                self.add_cmd(Push(1));
                self.add_cmd(Roll);
                self.swap();
                self.add_cmd(Sub);

                // Add 1?
                self.add_cmd(Push(1));
                self.add_cmd(Add);

                self.dup_at_depth();

                self.swap();
                self.add_cmd(Push(1));
                self.add_cmd(Add);

                self.add_expr(Expr::Comment(format!("-{:?}", var)));
            }
        }

        //     case "print_listC":
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_list"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["length"])

        //         label_index = goto_new_label(instrs, next_index) # move all elements to new array
        //         _, next_index = handle_smpl_instr(var_list, instrs, label_index, ["push", "1"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["sub"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["dup"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push", "-1"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["eq"])

        //         instrs[next_index][1].append("push 1")
        //         instrs[next_index][1].append("sub")

        //         swap(instrs, next_index)
        //         return_index, in_bounds_index = branch_new_labels(instrs, next_index)

        //         instrs[in_bounds_index][1].append("push 3")
        //         instrs[in_bounds_index][1].append("push -1")
        //         instrs[in_bounds_index][1].append("roll")
        //         swap(instrs, in_bounds_index)
        //         _, next_index = handle_smpl_instr(var_list, instrs, in_bounds_index, ["outC"])

        //         instrs[next_index][1].append("goto l" + str(label_index))

        //         _, next_index = handle_smpl_instr(var_list, instrs, return_index, ["pop"])

        //         index = next_index
        //         next_index = next_index

        //     case "print_listN":
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push",str(ord("["))])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["outC"])

        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_list"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["length"])

        //         label_index = goto_new_label(instrs, next_index) # move all elements to new array
        //         _, next_index = handle_smpl_instr(var_list, instrs, label_index, ["push", "1"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["sub"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["dup"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push", "-1"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["eq"])

        //         instrs[next_index][1].append("push 1")
        //         instrs[next_index][1].append("sub")

        //         swap(instrs, next_index)
        //         return_index, in_bounds_index = branch_new_labels(instrs, next_index)

        //         instrs[in_bounds_index][1].append("push 3")
        //         instrs[in_bounds_index][1].append("push -1")
        //         instrs[in_bounds_index][1].append("roll")
        //         swap(instrs, in_bounds_index)
        //         _, next_index = handle_smpl_instr(var_list, instrs, in_bounds_index, ["outN"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push",str(ord(","))])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["outC"])

        //         instrs[next_index][1].append("goto l" + str(label_index))

        //         _, next_index = handle_smpl_instr(var_list, instrs, return_index, ["pop"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push",str(ord("]"))])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["outC"])

        //         index = next_index
        //         next_index = next_index

        //     case "readC_until":
        //         label_index = goto_new_label(instrs, index)
        //         _, next_index = handle_smpl_instr(var_list, instrs, label_index, ["inC"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["dup"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push", l[1]])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["eq"])
        //         instrs[next_index][1].append("push 1")
        //         instrs[next_index][1].append("sub")
        //         swap(instrs, next_index)
        //         done_index, append_index = branch_new_labels(instrs, next_index)

        //         dup_value_x_deep(instrs, append_index, 3)
        //         swap(instrs, append_index)
        //         instrs[append_index][1].append("push 1")
        //         instrs[append_index][1].append("add")
        //         _, next_index = handle_smpl_instr(var_list, instrs, append_index, ["append"])
        //         instrs[next_index][1].append("push 3")
        //         instrs[next_index][1].append("push -1")
        //         instrs[next_index][1].append("roll")
        //         instrs[next_index][1].append("pop")
        //         instrs[next_index][1].append("push 1")
        //         instrs[next_index][1].append("sub")

        //         instrs[next_index][1].append("goto l" + str(label_index))

        //         _, next_index = handle_smpl_instr(var_list, instrs, done_index, ["pop"])

        //         index = next_index
        //         next_index = next_index

        //     case "readlines":
        //         label_index = goto_new_label(instrs, index)
        //         _, next_index = handle_smpl_instr(var_list, instrs, label_index, ["inC"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["dup"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push", "-1"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["eq"])
        //         instrs[next_index][1].append("push 1")
        //         instrs[next_index][1].append("sub")
        //         swap(instrs, next_index)
        //         done_index, append_index = branch_new_labels(instrs, next_index)

        //         _, next_index = handle_smpl_instr(var_list, instrs, append_index, ["push","-1"])

        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["append"])

        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["readC_until", "10"]) # read till newline

        //         instrs[next_index][1].append("push 3")
        //         instrs[next_index][1].append("push -1")
        //         instrs[next_index][1].append("roll")
        //         swap(instrs, next_index)

        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["append"])

        //         instrs[next_index][1].append("goto l" + str(label_index))

        //         _, next_index = handle_smpl_instr(var_list, instrs, done_index, ["pop"])

        //         index = next_index
        //         next_index = next_index

        //     case "printC_list_of_list":
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_list"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["length"])

        //         label_index = goto_new_label(instrs, next_index) # move all elements to new array
        //         _, next_index = handle_smpl_instr(var_list, instrs, label_index, ["push", "1"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["sub"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["dup"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push", "-1"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["eq"])

        //         instrs[next_index][1].append("push 1")
        //         instrs[next_index][1].append("sub")

        //         swap(instrs, next_index)
        //         return_index, in_bounds_index = branch_new_labels(instrs, next_index)

        //         instrs[in_bounds_index][1].append("push 3")
        //         instrs[in_bounds_index][1].append("push -1")
        //         instrs[in_bounds_index][1].append("roll")
        //         swap(instrs, in_bounds_index)
        //         _, next_index = handle_smpl_instr(var_list, instrs, in_bounds_index, ["print_listC"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["push","10"])
        //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["outC"])

        //         instrs[next_index][1].append("goto l" + str(label_index))

        //         _, next_index = handle_smpl_instr(var_list, instrs, return_index, ["pop"])

        //         index = next_index
        //         next_index = next_index

        //     case default:
        //         print ("Did not find", l)
    }

    pub fn to_stk(executor: SmplExecutor) -> PietStackExecutor {
        let mut smpl_to_stk = SmplToStk {
            smpl_executor: executor,
            stk_executor: PietStackExecutor {
                blocks: HashMap::new(),
                block_index: HashMap::new(),
                stack: vec![],
                label: String::from("main"),
            },
        };

        let mut bi = 0;

        // Setup stack invariants
        smpl_to_stk
            .stk_executor
            .blocks
            .insert(String::from("main"), vec![Instr(Push(0)), Instr(Push(1))]);
        smpl_to_stk
            .stk_executor
            .block_index
            .insert(String::from("main"), bi);
        bi += 1;

        // Stack frame
        for (_, var) in smpl_to_stk
            .smpl_executor
            .variables
            .clone()
            .into_iter()
            .sorted_by(|(_, a), (_, b)| a.var_index.cmp(&b.var_index).reverse())
        {
            smpl_to_stk.add_var(var);
        }

        for (x, _) in smpl_to_stk
            .smpl_executor
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

            let v = smpl_to_stk.smpl_executor.blocks[&x.clone()].clone();

            if x.clone() != "main" {
                smpl_to_stk.stk_executor.label = x.clone();
                smpl_to_stk.stk_executor.blocks.insert(x.clone(), vec![]);
                smpl_to_stk.stk_executor.block_index.insert(x.clone(), bi);
                bi += 1;
            }

            for e in v.clone() {
                // smpl_to_stk.add_expr(Comment(format!("+{:?}", e.clone())));
                smpl_to_stk.handle_smpl_instr(e.clone());
                // smpl_to_stk.add_expr(Comment(format!("-{:?}", e)));
            }
        }

        smpl_to_stk.stk_executor.label = String::from("main");

        smpl_to_stk.stk_executor
    }
}
