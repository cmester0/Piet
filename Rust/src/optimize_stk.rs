use crate::piet_interpreter::CMD::*;
use crate::piet_interpreter::*;

use std::collections::HashMap;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct StackOptimizer {
    results: HashMap<String, (usize, Vec<CMD>)>,
    // Weight, Curr_Stack, Curr_instrs, Total_instrs
    heap: BinaryHeap<Reverse<(usize, CMD, Vec<isize>, Vec<CMD>)>>,
}

impl StackOptimizer {
    pub fn new(
    ) -> Self {
        StackOptimizer {
            results: HashMap::from([(String::from("0"), (2, vec![Push(1), Not]))]),
            heap: BinaryHeap::from([
                Reverse((1, Push(1), vec![], vec![Push(1)])),
                Reverse((2, Push(2), vec![], vec![Push(2)])),
                Reverse((3, Push(3), vec![], vec![Push(3)])),
                Reverse((5, Push(5), vec![], vec![Push(5)])),
            ]),
        }
    }
}

fn int_root(x: usize, n: u32) -> usize {
    if x <= 0 || n <= 1 {
        return x;
    }

    let mut l = 0;
    let mut r = x;
    while l <= r {
        let m = (l + r) / 2;
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

fn vec_to_str(v: Vec<isize>) -> String {
    v.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

impl StackOptimizer {
    fn evaluate(&mut self, cmd: CMD, stack: &mut Vec<isize>) -> () {
        match cmd {
            Push(1) => stack.push(1),
            Push(2) => stack.push(2),
            Push(3) => stack.push(3),
            Push(5) => stack.push(5),
            _ => cmd.interpret::<std::io::Stdin, std::io::Stdout>(stack, &mut None, &mut None),
        }
    }

    fn add_neighbors(&mut self, weight: usize, stack: Vec<isize>, total_instructions: Vec<CMD>) {
        let mut add_next_instr = |weight_change: usize, next_instr: CMD| {
            let mut cp = total_instructions.clone();
            cp.push(next_instr);
            self.heap.push(Reverse((
                weight + weight_change,
                next_instr,
                stack.clone(),
                cp,
            )));
        };

        add_next_instr(1, Push(1));
        add_next_instr(2, Push(2));
        add_next_instr(3, Push(3));
        add_next_instr(5, Push(5));
        add_next_instr(1, Pop);
        add_next_instr(1, Add);
        add_next_instr(1, Sub);
        add_next_instr(1, Mul);
        add_next_instr(1, Div);
        add_next_instr(1, Mod);
        add_next_instr(1, Dup);
        if stack.len() >= 2
            && (stack.len() as isize) - 2 >= stack[stack.len() - 2]
            && stack[stack.len() - 2] > 0
        {
            // only allow valid roll? (Otherwise is better pop, pop?)
            add_next_instr(1, Roll);
        }
    }

    pub fn optimize_stack(&mut self, search_stack: Vec<isize>) -> Vec<CMD> {
        let search_stack = vec_to_str(search_stack);
        while !self.results.contains_key(&search_stack) {
            let Reverse((weight, instructions, mut stack, total_instructions)) =
                self.heap.pop().unwrap();

            self.evaluate(instructions, &mut stack);

            if stack.len() > 0 && stack[stack.len() - 1] == 0 {
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
    pub fn optimize_number(&mut self, n: usize) -> Vec<CMD> {
        let mut instrs = vec![];
        let mut root = 1;
        // print ("Calculate int root", N)
        while int_root(n, root) > 173 {
            root += 1;
        }
        instrs.extend(self.optimize_stack(vec![int_root(n, root) as isize]));
        for _ in 0..(root - 1) {
            instrs.push(CMD::Dup);
        }
        for _ in 0..(root - 1) {
            instrs.push(CMD::Mul);
        }
        if n - int_root(n, root).pow(root as u32) > 0 {
            instrs.extend(self.optimize_number(n - int_root(n, root).pow(root as u32)));
            instrs.push(CMD::Add);
        }
        instrs
    }
}
