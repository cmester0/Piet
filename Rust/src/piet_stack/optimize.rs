use super::expr::Expr::{self, *};
use crate::piet_interpreter::CMD::{self, *};

impl super::PietStackExecutor {
    pub fn optimize(&mut self) {
        for (k, v) in self.blocks.clone() {
            let mut new_block = v.clone().into_iter().filter(|x| if let Comment(_) = x { false } else { true }).collect::<Vec<_>>();

            // Unfold and simplify
            let mut i = 0;
            while i < new_block.len() {
                if i+5 < new_block.len() {
                    if let [Instr(Push(a)), Instr(Push(b)), Instr(Push(2)), Instr(Push(-1 | 1)), Instr(Roll)] = new_block[i..i + 5] {
                        new_block.remove(i);
                        new_block.remove(i);
                        new_block.remove(i);
                        new_block.remove(i);
                        new_block.remove(i);

                        new_block.insert(i, Instr(Push(b)));
                        new_block.insert(i, Instr(Push(a)));

                        i = 0;
                        continue
                    }
                }

                if i+2 < new_block.len() {
                    if let [Instr(Push(a)), Instr(op @ (Pop | Not | Dup))] = new_block[i..i + 2] {
                        match op {
                            Pop => {
                                new_block.remove(i);
                                new_block.remove(i);
                            }
                            Not => {
                                new_block[i] = Instr(Push(if a == 0 { 1 } else { 0 }));
                                new_block.remove(i+1);
                            }
                            Dup => {
                                new_block[i+1] = Instr(Push(a));
                            }
                            _ => panic!(),
                        }

                        i = 0;
                        continue
                    }
                }

                if i+2 < new_block.len() {
                    if let [Instr(Push(0)), Instr(op @ Add)] = new_block[i..i + 2] {
                        new_block.remove(i);
                        new_block.remove(i);
                        
                        i = 0;
                        continue
                    }
                }

                if i+2 < new_block.len() {
                    if let [Instr(Push(1)), Instr(op @ Mul)] = new_block[i..i + 2] {
                        new_block.remove(i);
                        new_block.remove(i);

                        i = 0;
                        continue
                    }
                }

                if i+3 < new_block.len() {
                    if let [Instr(Push(b)), Instr(Push(a)), Instr(Roll)] = new_block[i..i + 3] {
                        if a % b == 0 {
                            new_block.remove(i);
                            new_block.remove(i);
                            new_block.remove(i);

                            i = 0;
                            continue
                        }
                    }
                }

                if i+6 < new_block.len() {
                    if let [Instr(Push(a)), Instr(Push(b)), Instr(Roll), Instr(Push(c)), Instr(Push(d)), Instr(Roll)] = new_block[i..i + 6] {
                        if a == c {
                            new_block.remove(i);
                            new_block.remove(i);
                            new_block.remove(i);

                            new_block[i+1] = Instr(Push(b+d));

                            i = 0;
                            continue
                        }
                    }
                }

                if i+3 < new_block.len() {
                    if let [Instr(Push(a)), Instr(Push(b)), Instr(op @ (Add | Mul | Mod | Div | Sub | Greater))] = new_block[i..i + 3] {
                        new_block.remove(i);
                        new_block.remove(i);
                        new_block.remove(i);
                        new_block.insert(i, Instr(Push(
                            match op {
                                Add => a + b,
                                Mul => a * b,
                                Mod => a.rem_euclid(b),
                                Div => a / b,
                                Sub => a - b,
                                Greater => if a > b { 1 } else { 0 },
                                _ => panic!()
                            }
                        )));

                        i = 0;
                        continue
                    }
                }

                if i+4 < new_block.len() {
                    if let [Instr(Push(a)), Instr(op1 @ (Add)), Instr(Push(b)), Instr(op2 @ (Add))] = new_block[i..i + 4] {
                        new_block.remove(i+1);
                        new_block.insert(i+2, Instr(Add));

                        i = 0;
                        continue
                    }
                }

                if 4 <= i {
                    if let Instr(Roll) = new_block[i] {
                        if let Instr(Push(mut a)) = new_block[i-1] {
                            if let Instr(Push(k)) = new_block[i-2] {
                                if (k as usize)+3 <= i {
                                    if new_block[i-2-(k as usize)..i-2].into_iter().all(|x| if let Instr(Push(_)) = x {true} else {false}) {
                                        let b = k;

                                        if a != 0 && b != 0 && a % b != 0 {
                                            // new_block.remove(i-2);
                                            // new_block.remove(i-2);
                                            // new_block.remove(i-2);

                                            // a = a.rem_euclid(b);
                                            // new_block[i-2-(k as usize)..i-2].rotate_left(a as usize);

                                            // i -= k as usize;

                                            // i = 0;
                                            // continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                i += 1;
            }

            // // Refold
            // i = new_block.len()-1;
            // loop {
            //     if i+2 < new_block.len() {
            //         if let [Instr(Push(a)), Instr(Push(b))] = new_block[i..i + 2] {
            //             if a == b {
            //                 new_block.remove(i+1);
            //                 new_block.insert(i+1, Instr(Dup));
            //                 continue
            //             }
            //         }
            //     }

            //     if i == 0 {
            //         break
            //     }
            //     i -= 1;
            // }

            self.blocks.insert(k, new_block);
        }
    }
}