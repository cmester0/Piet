use crate::piet_interpreter::CMD::*;
use crate::piet_interpreter::*;
use num::BigInt;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

pub struct StackOptimizer {
    results: HashMap<String, (usize, Vec<CMD>)>,
    // Weight, Curr_Stack, Curr_instrs, Total_instrs
    heap: BinaryHeap<Reverse<(usize, CMD, Vec<BigInt>, Vec<CMD>)>>,
}

impl StackOptimizer {
    pub fn new() -> Self {
        StackOptimizer {
            results: HashMap::from([(String::from("0"), (2, vec![Push(1.into()), Not]))]),
            heap: BinaryHeap::from([
                Reverse((1, Push(1.into()), vec![], vec![Push(1.into())])),
                Reverse((2, Push(2.into()), vec![], vec![Push(2.into())])),
                Reverse((3, Push(3.into()), vec![], vec![Push(3.into())])),
                Reverse((5, Push(5.into()), vec![], vec![Push(5.into())])),
            ]),
        }
    }
}

fn int_root(x: BigInt, n: u32) -> BigInt {
    if x <= 0.into() || n <= 1 {
        return x;
    }

    let mut l : BigInt = 0.into();
    let mut r : BigInt = x.clone();
    while l.clone() <= r.clone() {
        let two : BigInt = 2.into();
        let m : BigInt = (l.clone() + r.clone()) / two;
        if m.pow(n) < x {
            l = m + 1
        } else if m.pow(n) > x {
            r = m - 1
        } else {
            return m;
        }
    }

    r
}

fn vec_to_str(v: Vec<BigInt>) -> String {
    v.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

impl StackOptimizer {
    fn evaluate(&mut self, cmd: CMD, stack: &mut Vec<BigInt>) -> () {
        match cmd {
            Push(c) if c == 1.into() || c == 2.into() || c == 3.into() || c == 5.into() => {
                stack.push(c)
            }
            _ => cmd.interpret::<std::io::Stdin, std::io::Stdout>(stack, &mut None, &mut None),
        }
    }

    fn add_neighbors(&mut self, weight: usize, stack: Vec<BigInt>, total_instructions: Vec<CMD>) {
        let mut add_next_instr = |weight_change: usize, next_instr: CMD| {
            let mut cp = total_instructions.clone();
            cp.push(next_instr.clone());
            self.heap.push(Reverse((
                weight + weight_change,
                next_instr,
                stack.clone(),
                cp,
            )));
        };

        add_next_instr(1, Push(1.into()));
        add_next_instr(2, Push(2.into()));
        add_next_instr(3, Push(3.into()));
        add_next_instr(5, Push(5.into()));
        add_next_instr(1, Pop);
        add_next_instr(1, Add);
        add_next_instr(1, Sub);
        add_next_instr(1, Mul);
        add_next_instr(1, Div);
        add_next_instr(1, Mod);
        add_next_instr(1, Dup);
        if stack.len() >= 2
            && <isize as Into<BigInt>>::into(stack.len() as isize - 2) >= stack[stack.len() - 2]
            && stack[stack.len() - 2] > 0.into()
        {
            // only allow valid roll? (Otherwise is better pop, pop?)
            add_next_instr(1, Roll);
        }
    }

    pub fn optimize_stack(&mut self, search_stack: Vec<BigInt>) -> Vec<CMD> {
        let search_stack = vec_to_str(search_stack);
        while !self.results.contains_key(&search_stack) {
            let Reverse((weight, instructions, mut stack, total_instructions)) =
                self.heap.pop().unwrap();

            self.evaluate(instructions, &mut stack);

            if stack.len() > 0 && stack[stack.len() - 1] == 0.into() {
                continue;
            }

            let stk_str = vec_to_str(stack.clone());
            if !self.results.contains_key(&stk_str) {
                self.results
                    .insert(stk_str, (weight, total_instructions.clone()));
                self.add_neighbors(weight, stack, total_instructions.clone());
            }
        }

        self.results[&search_stack].1.clone()
    }

    // TODO: Handle negative numbers!
    pub fn optimize_number(&mut self, n: BigInt) -> Vec<CMD> {
        let mut instrs = vec![];
        let mut root = 1;
        // print ("Calculate int root", N)
        while int_root(n.clone(), root) > 173.into() {
            root += 1;
        }
        instrs.extend(self.optimize_stack(vec![int_root(n.clone(), root).into()]));
        for _ in 0..(root - 1) {
            instrs.push(CMD::Dup);
        }
        for _ in 0..(root - 1) {
            instrs.push(CMD::Mul);
        }
        if n.clone() - int_root(n.clone(), root).pow(root as u32) > 0.into() {
            instrs.extend(self.optimize_number(n.clone() - int_root(n.clone(), root).pow(root as u32)));
            instrs.push(CMD::Add);
        }
        instrs
    }
}
