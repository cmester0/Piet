use itertools::Itertools;

use super::Expr::{self as SmplExpr};
use super::SmplExecutor;
use crate::piet_stack::expr::Expr::{self, *};
use crate::smpl::*;
use crate::{
    piet_interpreter::CMD::{self, *},
    piet_stack::PietStackExecutor,
};
use std::collections::HashMap;

pub struct SmplToStk {
    smpl_executor: SmplExecutor,
    stk_executor: PietStackExecutor,
    var_index: HashMap<String, usize>,
}

impl SmplToStk {
    fn c_add(&mut self, n: isize) {
        self.add_cmd(Push(n));
        self.add_cmd(Add);
    }

    fn c_sub(&mut self, n: isize) {
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

    fn c_binop(&mut self, op: CMD) {
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

    fn add_var(&mut self, var_name: String, var_type: Variable) {
        // Set variable index
        // (var_index.len() - var_index[i]) is actual index
        self.var_index.insert(var_name, self.var_index.len());

        // Allocate empty variable
        self.add_cmd(Push(match var_type {
            Variable::NUM(_) => 0,
            Variable::LIST(_) => -1,
        }));

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
            SmplExpr::Eq => {
                self.swap();
                self.add_cmd(Push(3));
                self.add_cmd(Push(-1));
                self.add_cmd(Roll);
                self.c_eq();
                self.swap();
                self.add_cmd(Push(1));
                self.add_cmd(Sub);
            }
            SmplExpr::Instr(c) => match c {
                Push(n) => {
                    self.c_add(1);
                    self.add_cmd(Push(n));
                    self.swap();
                }
                Pop => {
                    self.swap();
                    self.add_cmd(Pop);
                    self.c_sub(1);
                }
                Not => {
                    self.swap();
                    self.add_cmd(Not);
                    self.swap();
                }
                op @ (Add | Greater | Sub | Div | Mod | Mul) => {
                    self.c_binop(op);
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
                in_cmd @ (InN | InC) => {
                    self.goto_new_label();

                    self.add_cmd(Push(-2));
                    self.add_cmd(Push(-3));
                    self.add_cmd(in_cmd);
                    self.swap();

                    self.add_cmd(Push(-3));
                    self.c_eq();

                    self.c_if(
                        |x| {
                            x.add_cmd(Push(3));
                            x.add_cmd(Push(1));
                            x.add_cmd(Roll);
                            x.add_cmd(Pop);
                            x.add_cmd(Push(1));
                            x.add_cmd(Add);
                        },
                        |x| {
                            x.add_cmd(Pop);
                        },
                    );
                }
                out_cmd @ (OutC | OutN) => {
                    self.swap();
                    self.add_cmd(out_cmd);
                    self.add_cmd(Push(1));
                    self.add_cmd(Sub);
                }
                Roll => {
                    self.dup_value_x_deep(2);
                    self.dup_value_x_deep(4);
                    self.add_cmd(Mod);

                    self.add_cmd(Push(3));
                    self.add_cmd(Add);
                    self.add_cmd(Push(1));
                    self.add_cmd(Roll);

                    self.dup_value_x_deep(2);
                    self.add_cmd(Mod);

                    self.swap();
                    self.add_cmd(Push(1));
                    self.add_cmd(Add);
                    self.swap();

                    self.add_cmd(Roll);
                    self.add_cmd(Push(2));
                    self.add_cmd(Sub);
                }
                Nop => {}
                Pointer | Switch => panic!("Unsupported instructions {:?}", c),
            },
            SmplExpr::Goto(l) => {
                self.add_expr(Goto(l.get_label_name()));
            }
            SmplExpr::Branch(a, b) => {
                self.add_cmd(Push(1));
                self.add_cmd(Sub);
                self.swap();
                self.add_expr(Branch(a.get_label_name(), b.get_label_name()));
            }
            SmplExpr::Debug => {
                self.add_expr(Debug);
            }
            SmplExpr::Comment(s) => {
                self.add_expr(Comment(s));
            }
            SmplExpr::Set(var) => {
                todo!()
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

            }
            SmplExpr::Get(var) => {
                todo!()

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
            }
        }


        // match l[0]:

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
                smpl_to_stk.add_expr(Comment(format!("+{:?}", e.clone())));
                smpl_to_stk.handle_smpl_instr(e.clone());
                smpl_to_stk.add_expr(Comment(format!("-{:?}", e)));
            }
        }

        smpl_to_stk.stk_executor.label = String::from("main");

        smpl_to_stk.stk_executor
    }
}
