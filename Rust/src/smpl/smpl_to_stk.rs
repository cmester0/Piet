use super::SmplExecutor;
use crate::piet_stack::expr::Expr::{self, *};
use super::Expr::{self as SmplExpr};
use crate::piet_stack::*;
use crate::{
    piet_interpreter::CMD::{self, *},
    piet_stack::PietStackExecutor,
};
use std::borrow::Borrow;
use std::collections::HashMap;
use crate::smpl::*;

pub struct SmplToStk {
    smpl_executor: SmplExecutor,
    stk_executor: PietStackExecutor,
    var_index: HashMap<String, usize>,
}

impl SmplToStk {
    fn c_add(&mut self, n : isize) {
        self.add_cmd(Push(n));
        self.add_cmd(Add);
    }

    fn c_sub(&mut self, n : isize) {
        self.add_cmd(Push(n));
        self.add_cmd(Sub);
    }

    fn c_eq(&mut self) {
        self.add_cmd(Sub);
        self.add_cmd(Dup);
        self.add_cmd(Mul);
        self.add_cmd(Push(0));
        self.add_cmd(Greater);
        self.add_cmd(Not);
    }

    fn c_binop(&mut self, op : CMD) {
        self.add_cmd(Push(3));
        self.add_cmd(Push(1));
        self.add_cmd(Roll);
        self.add_cmd(op);
        self.swap();
        self.add_cmd(Push(-1));
        self.add_cmd(Add);
    }
}

impl SmplToStk {
    fn add_cmd(&mut self, c: CMD) {
        self.stk_executor
            .blocks
            .get_mut(&self.stk_executor.label)
            .unwrap()
            .push(Instr(c));
    }

    fn add_cmds(&mut self, c: Vec<CMD>) {
        self.stk_executor
            .blocks
            .get_mut(&self.stk_executor.label)
            .unwrap()
            .extend(c.into_iter().map(Instr).collect::<Vec<_>>());
    }

    fn swap(&mut self) {
        self.add_cmds(vec![
            Push(2),
            Push(1),
            Roll]);
    }

    // fn dup_value_x_deep(x: isize) -> Vec<Expr> {
    //     vec![Push(x), Push(-1), Roll, Dup, Push(x + 1), Push(1), Roll]
    //         .into_iter()
    //         .map(Instr)
    //         .collect()
    // }

    pub fn add_var(&mut self, var_name: String, var_type: Variable) {
        // Set variable index
        // (var_index.len() - var_index[i]) is actual index
        self.var_index.insert(var_name, self.var_index.len());

        // Allocate empty variable
        self.add_cmd(Push(match var_type { Variable::NUM(_) => 0, Variable::LIST(_) => -1 }));

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

    pub fn handle_smpl_instr(&mut self, e: SmplExpr) {
        match e {
            SmplExpr::Eq => {
                self.swap();
                self.add_cmd(Push(3));
                self.add_cmd(Push(-1));
                self.add_cmd(Roll);
                self.c_eq();
                self.swap();
                self.add_cmd(Push(1));
                self.add_cmd(Sub);
            },
            SmplExpr::Instr(c) => {
                match c {
                    Push(n) => {
                        self.c_add(1);
                        self.add_cmd(Push(n));
                        self.swap();
                    },
                    Pop => {
                        self.swap();
                        self.add_cmd(Pop);
                        self.c_sub(1);
                    },
                    Not => {
                        self.swap();
                        self.add_cmd(Not);
                        self.swap();
                    },
                    Add | Greater | Sub | Div | Mod | Mul => {
                        self.c_binop(c);
                    }
                    Dup => {
                        self.swap();
                        self.add_cmd(Dup);
                        self.add_cmd(Push(3));
                        self.add_cmd(Push(-1));
                        self.add_cmd(Roll);
                        self.add_cmd(Push(1));
                        self.add_cmd(Add);
                    }
                    InN => {
                        let new_index = self.stk_executor.block_index.len().clone();
                        let new_block_label = format!("l{}", new_index);
                        // let new_block_label : &'a str = new_block_label.as_str();

                        // self.stk_executor
                        //     .blocks
                        //     .get_mut(self.stk_executor.label)
                        //     .unwrap()
                        //     .push(Goto(new_block_label));

                        // self.stk_executor
                        //     .blocks
                        //     .insert(new_block_label.as_str(), vec![]);
                        
                        // self.stk_executor.label = new_block_label.as_str();
                    }
                    _ => todo!(),
                }
            },
            _ => todo!(),
        }
    // match l[0]:
    //     case "label":
    //         index = len(instrs)
    //         next_index = len(instrs)
    //         instrs.append((l[1], []))

    //     case "inN":
    //         label_index = len(instrs)
    //         new_label = "l" + str(label_index)
    //         instrs[index][1].append("goto " + new_label)

    //         instrs.append((new_label, []))
    //         instrs[label_index][1].append("push -2")
    //         instrs[label_index][1].append("push -3")
    //         instrs[label_index][1].append("inN")
    //         swap(instrs, label_index)

    //         # Is it -3 ?
    //         instrs[label_index][1].append("push -3")
    //         eq(instrs, label_index)

    //         succ_label_index, fail_label_index = branch_new_labels(instrs, label_index)

    //         continue_label_index = len(instrs)
    //         continue_new_label = "l" + str(continue_label_index)
    //         instrs.append((continue_new_label, []))

    //         instrs[succ_label_index][1].append("push 3")
    //         instrs[succ_label_index][1].append("push 1")
    //         instrs[succ_label_index][1].append("roll")
    //         instrs[succ_label_index][1].append("pop")
    //         instrs[succ_label_index][1].append("push 1")
    //         instrs[succ_label_index][1].append("add")
    //         instrs[succ_label_index][1].append("goto " + continue_new_label)

    //         instrs[fail_label_index][1].append("pop")
    //         instrs[fail_label_index][1].append("goto " + continue_new_label)

    //         index = fail_label_index
    //         next_index = continue_label_index

    //     case "inC":
    //         label_index = len(instrs)
    //         new_label = "l" + str(label_index)
    //         instrs[index][1].append("goto " + new_label)

    //         instrs.append((new_label, []))
    //         instrs[label_index][1].append("push -2")
    //         instrs[label_index][1].append("push -3")
    //         instrs[label_index][1].append("inC")
    //         swap(instrs, label_index)

    //         # Is it -3 ?
    //         instrs[label_index][1].append("push -3")
    //         eq(instrs, label_index)

    //         succ_label_index, fail_label_index = branch_new_labels(instrs, label_index)

    //         continue_label_index = len(instrs)
    //         continue_new_label = "l" + str(continue_label_index)
    //         instrs.append((continue_new_label, []))

    //         instrs[succ_label_index][1].append("push 3")
    //         instrs[succ_label_index][1].append("push 1")
    //         instrs[succ_label_index][1].append("roll")
    //         instrs[succ_label_index][1].append("pop")
    //         instrs[succ_label_index][1].append("push 1")
    //         instrs[succ_label_index][1].append("add")
    //         instrs[succ_label_index][1].append("goto " + continue_new_label)

    //         instrs[fail_label_index][1].append("pop")
    //         instrs[fail_label_index][1].append("goto " + continue_new_label)

    //         index = fail_label_index
    //         next_index = continue_label_index

    //     case "goto":
    //         instrs[index][1].append("goto " + l[1])

    //     case "branch":
    //         instrs[index][1].append("push 1")
    //         instrs[index][1].append("sub")
    //         swap(instrs, index)
    //         instrs[index][1].append("branch " + l[1] + " " + l[2])

    //     case "set":
    //         assert (len(l) == 2) # set

    //         var_index = 0
    //         for i, x in enumerate(var_list):
    //             if x == l[1]:
    //                 var_index = i
    //                 break
    //         else:
    //             print ("Variable", l[1], "was not defined")
    //             exit(1)

    //         new_index = get_offset_for_var_index(instrs, index, var_index)
    //         swap(instrs, new_index)
    //         instrs[new_index][1].append("dup")

    //         instrs[new_index][1].append("push 3")
    //         instrs[new_index][1].append("push 1")
    //         instrs[new_index][1].append("roll")
    //         swap(instrs, new_index)
    //         instrs[new_index][1].append("sub")

    //         instrs[new_index][1].append("push 1")
    //         instrs[new_index][1].append("add")
    //         # Add 1?
    //         # instrs[new_index][1].append("push 1")
    //         # instrs[new_index][1].append("add")

    //         instrs[new_index][1].append("push 3")
    //         instrs[new_index][1].append("push -1")
    //         instrs[new_index][1].append("roll")

    //         swap(instrs, new_index)

    //         swap_at_depth(instrs, new_index)

    //         instrs[new_index][1].append("pop")
    //         instrs[new_index][1].append("push 1")
    //         instrs[new_index][1].append("sub")

    //         index = new_index
    //         next_index = new_index

    //     case "get":
    //         assert (len(l) == 2) # get

    //         var_index = 0
    //         for i, x in enumerate(var_list):
    //             if x == l[1]:
    //                 var_index = i
    //                 break
    //         else:
    //             print ("Variable", l[1], "was not defined")
    //             exit(1)

    //         new_index = get_offset_for_var_index(instrs, index, var_index)
    //         swap(instrs, new_index)
    //         instrs[new_index][1].append("dup")

    //         instrs[new_index][1].append("push 3")
    //         instrs[new_index][1].append("push 1")
    //         instrs[new_index][1].append("roll")
    //         swap(instrs, new_index)
    //         instrs[new_index][1].append("sub")

    //         # Add 1?
    //         instrs[new_index][1].append("push 1")
    //         instrs[new_index][1].append("add")

    //         dup_at_depth(instrs, new_index)

    //         swap(instrs, new_index)
    //         instrs[new_index][1].append("push 1")
    //         instrs[new_index][1].append("add")

    //         index = new_index
    //         next_index = new_index

    //     case "append":
    //         assert (len(l) == 1) # append

    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("dup")

    //         # Is it -1?
    //         instrs[next_index][1].append("push -1")
    //         eq(instrs, next_index)

    //         suc_label_index, fail_label_index = branch_new_labels(instrs, next_index)

    //         ####################################
    //         # Success block = Initialize alloc #
    //         ####################################

    //         instrs[suc_label_index][1].append("pop")
    //         instrs[suc_label_index][1].append("push 3")
    //         swap(instrs, suc_label_index)
    //         _, next_index = handle_smpl_instr(var_list, instrs, suc_label_index, ["malloc"])

    //         instrs[next_index][1].append("dup")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         dup_at_depth(instrs, next_index)

    //         instrs[next_index][1].append("push 3")
    //         instrs[next_index][1].append("sub")

    //         swap(instrs, next_index)

    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("dup")

    //         instrs[next_index][1].append("push 4")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("roll")
    //         swap(instrs, next_index)

    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         instrs[next_index][1].append("push 0")

    //         dup_value_x_deep(instrs, next_index, 3)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         instrs[next_index][1].append("push 1")

    //         instrs[next_index][1].append("push 5")
    //         instrs[next_index][1].append("push 3")
    //         instrs[next_index][1].append("roll")

    //         instrs[next_index][1].append("push 3")
    //         instrs[next_index][1].append("add")

    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])
    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

    //         instrs[next_index][1].append("push 3")
    //         instrs[next_index][1].append("push -1")
    //         instrs[next_index][1].append("roll")

    //         instrs[next_index][1].append("goto l" + str(fail_label_index))

    //         ###########################################
    //         # End of Success block = Initialize alloc #
    //         ###########################################

    //         instrs[fail_label_index][1].append("push 3")
    //         instrs[fail_label_index][1].append("push 1")
    //         instrs[fail_label_index][1].append("roll")

    //         dup_value_x_deep(instrs, fail_label_index, 3)
    //         swap(instrs, fail_label_index)
    //         instrs[fail_label_index][1].append("push 1")
    //         instrs[fail_label_index][1].append("add")

    //         dup_value_x_deep(instrs, fail_label_index, 2)
    //         swap(instrs, fail_label_index)

    //         instrs[fail_label_index][1].append("push 1")
    //         instrs[fail_label_index][1].append("add")

    //         _, next_index = handle_smpl_instr(var_list, instrs, fail_label_index, ["get_heap"])

    //         instrs[next_index][1].append("push 3")
    //         instrs[next_index][1].append("push -1")
    //         instrs[next_index][1].append("roll")

    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")
    //         swap(instrs, next_index)

    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_heap"])

    //         dup_value_x_deep(instrs, next_index, 3)
    //         dup_value_x_deep(instrs, next_index, 3)
    //         instrs[next_index][1].append("greater")

    //         in_bounds_index, realloc_index = branch_new_labels(instrs, next_index)

    //         continue_label_index = len(instrs)
    //         continue_new_label = "l" + str(continue_label_index)
    //         instrs.append((continue_new_label, []))

    //         ###################
    //         # in bounds block #
    //         ###################

    //         instrs[in_bounds_index][1].append("push 3")
    //         instrs[in_bounds_index][1].append("push 1")
    //         instrs[in_bounds_index][1].append("roll")
    //         swap(instrs, in_bounds_index)

    //         instrs[in_bounds_index][1].append("pop")
    //         instrs[in_bounds_index][1].append("push 1")
    //         instrs[in_bounds_index][1].append("add")
    //         instrs[in_bounds_index][1].append("dup")

    //         instrs[in_bounds_index][1].append("push 3")
    //         instrs[in_bounds_index][1].append("push 2")
    //         instrs[in_bounds_index][1].append("roll")

    //         next_index = in_bounds_index
    //         dup_value_x_deep(instrs, next_index, 5)
    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")
    //         swap(instrs, next_index)

    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

    //         dup_value_x_deep(instrs, next_index, 4)
    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         instrs[next_index][1].append("push 3")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("roll")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")
    //         instrs[next_index][1].append("add")
    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("sub")

    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

    //         instrs[next_index][1].append("goto " + continue_new_label)

    //         ##########################
    //         # END of in bounds block #
    //         ##########################

    //         #################
    //         # realloc block #
    //         #################

    //         # instrs[realloc_index][1].append("push 3")
    //         swap(instrs, realloc_index)
    //         instrs[realloc_index][1].append("pop")
    //         swap(instrs, realloc_index)
    //         instrs[realloc_index][1].append("push 2")
    //         instrs[realloc_index][1].append("mul")
    //         instrs[realloc_index][1].append("dup")
    //         instrs[realloc_index][1].append("push 2")
    //         instrs[realloc_index][1].append("add")

    //         instrs[realloc_index][1].append("push 3")
    //         instrs[realloc_index][1].append("push 2")
    //         instrs[realloc_index][1].append("roll")

    //         _, next_index = handle_smpl_instr(var_list, instrs, realloc_index, ["malloc"])

    //         # _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get", l[1]])

    //         dup_value_x_deep(instrs, next_index, 4)
    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         instrs[next_index][1].append("dup")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         dup_at_depth(instrs, next_index)

    //         instrs[next_index][1].append("push 2")
    //         instrs[next_index][1].append("sub")
    //         dup_value_x_deep(instrs, next_index, 4)
    //         instrs[next_index][1].append("sub")

    //         swap(instrs, next_index)

    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         # _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set", l[1]])

    //         instrs[next_index][1].append("push 6")
    //         instrs[next_index][1].append("push -1")
    //         instrs[next_index][1].append("roll")
    //         instrs[next_index][1].append("pop")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("sub")
    //         swap(instrs, next_index)

    //         instrs[next_index][1].append("push 5")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("roll")

    //         dup_value_x_deep(instrs, next_index, 2)
    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")
    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_heap"])
    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 2")
    //         instrs[next_index][1].append("mul")
    //         swap(instrs, next_index)

    //         # _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get", l[1]])

    //         dup_value_x_deep(instrs, next_index, 6)
    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("dup")

    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         instrs[next_index][1].append("push 3")
    //         instrs[next_index][1].append("push 2")
    //         instrs[next_index][1].append("roll")

    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_heap"])

    //         instrs[next_index][1].append("push 0")
    //         swap(instrs, next_index)

    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         dup_value_x_deep(instrs, next_index, 7)
    //         instrs[next_index][1].append("push 5")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("roll")

    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         _, done_index = handle_smpl_instr(var_list, instrs, next_index, ["copy_memory"])

    //         instrs[done_index][1].append("push 5")
    //         instrs[done_index][1].append("push -1")
    //         instrs[done_index][1].append("roll")

    //         # Done index
    //         # _, next_index = handle_smpl_instr(var_list, instrs, done_index, ["get", l[1]])
    //         next_index = done_index
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")
    //         swap(instrs, next_index)

    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])
    //         instrs[next_index][1].append("push 3")
    //         instrs[next_index][1].append("push -1")
    //         instrs[next_index][1].append("roll")
    //         instrs[next_index][1].append("pop")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("sub")

    //         instrs[next_index][1].append("goto l" + str(in_bounds_index))

    //         ########################
    //         # end of realloc block #
    //         ########################

    //         index = next_index
    //         next_index = continue_label_index

    //     case "get_heap":
    //         assert (len(l) == 1) # get_heap
    //         instrs[index][1].append("dup")
    //         instrs[index][1].append("push 1")
    //         instrs[index][1].append("add")

    //         dup_at_depth(instrs, index)
    //         dup_value_x_deep(instrs, index, 2)
    //         instrs[index][1].append("add")

    //         instrs[index][1].append("push 3")
    //         instrs[index][1].append("push -1")
    //         instrs[index][1].append("roll")
    //         instrs[index][1].append("sub")

    //         dup_at_depth(instrs, index)
    //         swap(instrs, index)

    //         index = index
    //         next_index = index

    //     case "set_heap":
    //         assert (len(l) == 1) # set_heap

    //         instrs[index][1].append("dup")
    //         instrs[index][1].append("push 1")
    //         instrs[index][1].append("add")

    //         dup_at_depth(instrs, index)
    //         dup_value_x_deep(instrs, index, 2)
    //         instrs[index][1].append("add")
    //         # instrs[index][1].append("push 1")
    //         # instrs[index][1].append("add")

    //         instrs[index][1].append("push 3")
    //         instrs[index][1].append("push -1")
    //         instrs[index][1].append("roll")
    //         instrs[index][1].append("sub")

    //         instrs[index][1].append("push 3")
    //         instrs[index][1].append("push -1")
    //         instrs[index][1].append("roll")

    //         swap(instrs, index)
    //         swap_at_depth(instrs, index)

    //         instrs[index][1].append("pop")
    //         instrs[index][1].append("push 2")
    //         instrs[index][1].append("sub")

    //         index = index
    //         next_index = index

    //     case "get_list":
    //         _, next_index = handle_smpl_instr(var_list, instrs, index, ["dup"])
    //         _, next_index = handle_smpl_instr(var_list, instrs, index, ["length"])

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

    //         _, next_index = handle_smpl_instr(var_list, instrs, in_bounds_index, ["dup"])

    //         dup_value_x_deep(instrs, next_index, 4)
    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_elem"])

    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 4")
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("roll")

    //         instrs[next_index][1].append("goto l" + str(label_index))

    //         _, next_index = handle_smpl_instr(var_list, instrs, return_index, ["pop"])

    //         index = next_index
    //         next_index = next_index

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

    //     case "outC":
    //         swap(instrs, index)
    //         instrs[index][1].append("outC")
    //         instrs[index][1].append("push 1")
    //         instrs[index][1].append("sub")

    //     case "outN":
    //         swap(instrs, index)
    //         instrs[index][1].append("outN")
    //         instrs[index][1].append("push 1")
    //         instrs[index][1].append("sub")


    //     case "roll":
    //         dup_value_x_deep(instrs, index, 2)
    //         dup_value_x_deep(instrs, index, 4)
    //         instrs[index][1].append("mod")

    //         instrs[index][1].append("push 3")
    //         instrs[index][1].append("add")
    //         instrs[index][1].append("push 1")
    //         instrs[index][1].append("roll")

    //         dup_value_x_deep(instrs, index, 2)
    //         instrs[index][1].append("mod")

    //         swap(instrs, index)
    //         instrs[index][1].append("push 1")
    //         instrs[index][1].append("add")
    //         swap(instrs, index)

    //         instrs[index][1].append("roll")
    //         instrs[index][1].append("push 2")
    //         instrs[index][1].append("sub")

    //     case "malloc":
    //         instrs[index][1].append("dup")

    //         instrs[index][1].append("push 3")
    //         instrs[index][1].append("push -1")
    //         instrs[index][1].append("roll")

    //         instrs[index][1].append("dup")
    //         instrs[index][1].append("dup")

    //         instrs[index][1].append("push 4")
    //         instrs[index][1].append("push -1")
    //         instrs[index][1].append("roll")
    //         instrs[index][1].append("add")
    //         instrs[index][1].append("push 1")
    //         instrs[index][1].append("add")

    //         swap(instrs, index)
    //         instrs[index][1].append("dup")

    //         label_index = goto_new_label(instrs, index)
    //         instrs[label_index][1].append("dup")
    //         instrs[label_index][1].append("push 0")
    //         instrs[label_index][1].append("greater")
    //         loop_index, roll_index = branch_new_labels(instrs, label_index)

    //         instrs[loop_index][1].append("push 0")

    //         instrs[loop_index][1].append("push 4")
    //         instrs[loop_index][1].append("push 1")
    //         instrs[loop_index][1].append("roll")

    //         instrs[loop_index][1].append("push 1")
    //         instrs[loop_index][1].append("sub")
    //         instrs[loop_index][1].append("goto " + "l" + str(label_index))

    //         instrs[roll_index][1].append("pop")
    //         instrs[roll_index][1].append("roll")

    //         dup_value_x_deep(instrs, roll_index, 2)

    //         # # Add 1 extra??
    //         instrs[roll_index][1].append("push 1")
    //         instrs[roll_index][1].append("add")

    //         dup_at_depth(instrs, roll_index)
    //         instrs[roll_index][1].append("add")

    //         dup_value_x_deep(instrs, roll_index, 2)
    //         # # Add 1 extra??
    //         instrs[roll_index][1].append("push 1")
    //         instrs[roll_index][1].append("add")
    //         swap_at_depth(instrs, roll_index)
    //         instrs[roll_index][1].append("pop")

    //         # Decrement stack size
    //         instrs[roll_index][1].append("push 1")
    //         instrs[roll_index][1].append("sub")

    //         index = roll_index
    //         next_index = roll_index

    //     case "length":
    //         assert (len(l) == 1) # length
    //         swap(instrs, index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")
    //         swap(instrs, next_index)
    //         index, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get_heap"])

    //     case "copy_memory":
    //         label_index = goto_new_label(instrs, index) # move all elements to new array

    //         dup_value_x_deep(instrs, label_index, 3)
    //         dup_value_x_deep(instrs, label_index, 3)
    //         eq(instrs, label_index)

    //         done_index, loop_index = branch_new_labels(instrs, label_index)

    //         dup_value_x_deep(instrs, loop_index, 4)
    //         dup_value_x_deep(instrs, loop_index, 3)
    //         instrs[loop_index][1].append("add")
    //         instrs[loop_index][1].append("push 2")
    //         instrs[loop_index][1].append("add")

    //         swap(instrs, loop_index)
    //         instrs[loop_index][1].append("push 1")
    //         instrs[loop_index][1].append("add")

    //         _, next_index = handle_smpl_instr(var_list, instrs, loop_index, ["get_heap"])

    //         # _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["get", l[1]])
    //         dup_value_x_deep(instrs, next_index, 6)
    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")

    //         swap(instrs, next_index)
    //         dup_value_x_deep(instrs, next_index, 4)
    //         instrs[next_index][1].append("add")
    //         instrs[next_index][1].append("push 2")
    //         instrs[next_index][1].append("add")
    //         swap(instrs, next_index)
    //         _, next_index = handle_smpl_instr(var_list, instrs, next_index, ["set_heap"])

    //         swap(instrs, next_index)
    //         instrs[next_index][1].append("push 1")
    //         instrs[next_index][1].append("add")
    //         swap(instrs, next_index)

    //         instrs[next_index][1].append("goto l" + str(label_index))

    //         index = next_index
    //         next_index = done_index

    //     case "debug":
    //         instrs[index][1].append("debug")

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
            var_index: HashMap::new(),
        };

        // Setup stack invariants
        smpl_to_stk
            .stk_executor
            .blocks
            .insert(String::from("main"), vec![Instr(Push(0)), Instr(Push(1))]);
        smpl_to_stk.stk_executor.block_index.insert(String::from("main"), 0);

        // var_list = add_var(var_list, "__R0__", "num")
        // var_list = add_var(var_list, "__R1__", "num")
        // var_list = add_var(var_list, "__R2__", "num")
        // var_list = add_var(var_list, "__R3__", "num")
        // var_list = add_var(var_list, "__R4__", "num")
        // var_list = add_var(var_list, "__R5__", "num")
        // var_list = add_var(var_list, "__R6__", "num")
        // var_list = add_var(var_list, "__R7__", "num")

        // Stack frame
        for (var_name, var_type) in smpl_to_stk.smpl_executor.variables.clone() {
            smpl_to_stk.add_var(var_name, var_type);
        }

        for (x, v) in smpl_to_stk.smpl_executor.blocks.clone() {
            for e in v.clone() {
                smpl_to_stk.handle_smpl_instr(e);
            }
        }
    // for l in inp_lines[inp_line_index:]:
    //     if len(l) == 0 or l[0] == "#":
    //         continue
    //     instrs[index][1].append("#+" + " ".join(l))
    //     last_index, index = handle_smpl_instr(var_list, instrs, index, l)
    //     instrs[last_index][1].append("#-" + " ".join(l))


        smpl_to_stk.stk_executor
    }
}
