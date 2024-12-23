use super::expr::Expr::*;
use crate::piet_interpreter::CMD::*;
use num::traits::Euclid;
use num::ToPrimitive;

impl super::PietStackExecutor {
    pub fn optimize(&mut self) {
        for (k, v) in self.blocks.clone() {
            let mut new_block = v
                .clone()
                .into_iter()
                .filter(|x| if let Comment(_) = x { false } else { true })
                .collect::<Vec<_>>();

            // Unfold and simplify
            let mut i = 0;
            while i < new_block.len() {
                if i + 2 < new_block.len() {
                    if let [Instr(Push(a)), Instr(op @ (Pop | Not | Dup))] = &new_block[i..i + 2] {
                        match op {
                            Pop => {
                                new_block.remove(i);
                                new_block.remove(i);
                            }
                            Not => {
                                new_block[i] = Instr(Push(if a.clone() == 0.into() {
                                    1.into()
                                } else {
                                    0.into()
                                }));
                                new_block.remove(i + 1);
                            }
                            Dup => {
                                new_block[i + 1] = Instr(Push(a.clone()));
                            }
                            _ => panic!(),
                        }

                        i = 0;
                        continue;
                    }
                }

                if i + 2 < new_block.len() {
                    if let [Instr(Push(c)), Instr(_op @ (Add | Sub))] =
                        &new_block[i.to_usize().unwrap()..i.to_usize().unwrap() + 2]
                    {
                        if c.clone() == 0.into() {
                            new_block.remove(i);
                            new_block.remove(i);

                            i = 0;
                            continue;
                        }
                    }
                }

                if i + 2 < new_block.len() {
                    if let [Instr(Push(c)), Instr(_op @ (Mul | Div))] =
                        &new_block[i.to_usize().unwrap()..i.to_usize().unwrap() + 2]
                    {
                        if c.clone() == 1.into() {
                            new_block.remove(i);
                            new_block.remove(i);

                            i = 0;
                            continue;
                        }
                    }
                }

                if i + 3 < new_block.len() {
                    if let [Instr(Push(b)), Instr(Push(a)), Instr(Roll)] =
                        &new_block[i.to_usize().unwrap()..i.to_usize().unwrap() + 3]
                    {
                        if b.clone() != 0.into() && a.clone() % b.clone() == 0.into() {
                            new_block.remove(i);
                            new_block.remove(i);
                            new_block.remove(i);

                            i = 0;
                            continue;
                        }
                    }
                }

                // if i + 6 < new_block.len() {
                //     if let [Instr(Push(a)), Instr(Push(b)), Instr(Roll), Instr(Push(c)), Instr(Push(d)), Instr(Roll)] =
                //         &new_block[i..i + 6]
                //     {
                //         if (a == &3.into()) && (b == &1.into()) && (c == &2.into()) && (d == &1.into() || d == &(-1).into()) {
                //             new_block[i] = Instr(Push(2.into()));
                //             new_block[i + 3] = Instr(Push(3.into()));
                //             new_block[i + 4] = Instr(Push((-1).into()));

                //             i = 0;
                //             continue;
                //         }
                //     }
                // }

                // if i + 6 < new_block.len() {
                //     if let [Instr(Push(3)), Instr(Push(-1 | 2)), Instr(Roll), Instr(Push(2)), Instr(Push(1 | -1)), Instr(Roll)] =
                //         new_block[i..i + 6]
                //     {
                //         new_block[i] = Instr(Push(2));
                //         new_block[i + 1] = Instr(Push(1));
                //         new_block[i + 3] = Instr(Push(3));
                //         new_block[i + 4] = Instr(Push(1));

                //         i = 0;
                //         continue;
                //     }
                // }

                // if i + 6 < new_block.len() {
                //     if let [Instr(Push(a)), Instr(Push(b)), Instr(Roll), Instr(Push(c)), Instr(Push(d)), Instr(Roll)] =
                //         new_block[i..i + 6]
                //     {
                //         if a == c {
                //             new_block.remove(i);
                //             new_block.remove(i);
                //             new_block.remove(i);

                //             new_block[i + 1] = Instr(Push(b + d));

                //             i = 0;
                //             continue;
                //         }
                //     }

                // }

                // if i + 4 < new_block.len() {
                //     if let [Instr(Dup), Instr(Push(2)), Instr(Push(1 | -1)), Instr(Roll)] =
                //         new_block[i..i + 4]
                //     {
                //         new_block.remove(i + 1);
                //         new_block.remove(i + 1);
                //         new_block.remove(i + 1);

                //         i = 0;
                //         continue;
                //     }
                // }

                // if i + 9 < new_block.len() {
                //     if let [Instr(Push(2)), Instr(Push(1 | -1)), Instr(Roll), Instr(Dup), Instr(Push(3)), Instr(Push(-1)), Instr(Roll), Instr(Push(k)), Instr(op @ (Add | Mul | Mod | Div | Sub | Greater))] =
                //         new_block[i..i + 9]
                //     {
                //         new_block.remove(i + 7);
                //         new_block.remove(i + 7);
                //         new_block.insert(i, Instr(op));
                //         new_block.insert(i, Instr(Push(k)));

                //         i = 0;
                //         continue;
                //     }
                // }

                // if i + 7 < new_block.len() {
                //     if let [Instr(Push(2)), Instr(Push(1 | -1)), Instr(Roll), Instr(Push(_k)), Instr(Push(2)), Instr(Push(1 | -1)), Instr(Roll)] =
                //         new_block[i..i + 7]
                //     {
                //         new_block.remove(i);
                //         new_block.remove(i);
                //         new_block.remove(i);
                //         new_block[i + 1] = Instr(Push(3.into()));
                //         new_block[i + 2] = Instr(Push((-1).into()));

                //         i = 0;
                //         continue;
                //     }
                // }

                if i + 3 < new_block.len() {
                    if let Instr(Push(a)) = new_block[i].clone() {
                        if let Instr(Push(b)) = new_block[i+1].clone() {
                            if let Instr(op @ (Add | Mul | Mod | Div | Sub | Greater)) = new_block[i+2].clone() {
                                new_block.remove(i);
                                new_block.remove(i);
                                new_block[i] = Instr(Push(match op {
                                    Add => a + b,
                                    Mul => a * b,
                                    Mod => a.rem_euclid(&b),
                                    Div => a / b,
                                    Sub => a - b,
                                    Greater => if a > b { 1 } else { 0 }.into(),
                                    _ => panic!(),
                                }));

                                i = 0;
                                continue;
                            }
                        }
                    }
                }

                // if i + 2 < new_block.len() {
                //     if let [Instr(Push(a)), Instr(Sub)] = &new_block[i..i + 2] {
                //         new_block[i] = Instr(Push(-a));
                //         new_block[i + 1] = Instr(Add);

                //         i = 0;
                //         continue;
                //     }
                // }

                // Add assoc
                if i + 4 < new_block.len() {
                    if let [Instr(_op1 @ Add), Instr(Push(_b)), Instr(_op2 @ Add)] =
                        &new_block[i..i + 4]
                    {
                        new_block.remove(i);
                        new_block.insert(i + 1, Instr(Add));

                        i = 0;
                        continue;
                    }
                }

                if 4 <= i {
                    if let Instr(Roll) = new_block[i] {
                        if let Instr(Push(mut a)) = new_block[i - 1].clone() {
                            if let Instr(Push(k)) = new_block[i - 2].clone() {
                                if k > 0.into() && (k.to_usize().unwrap()) + 3 <= i {
                                    if new_block[i - 2 - (k.to_usize().unwrap())..i - 2]
                                        .into_iter()
                                        .all(|x| if let Instr(Push(_)) = x { true } else { false })
                                    {
                                        let b = k.clone();

                                        if a != 0.into() {
                                            new_block.remove(i - 2);
                                            new_block.remove(i - 2);
                                            new_block.remove(i - 2);

                                            a = a.rem_euclid(&b);

                                            new_block[i - 2 - (k.to_usize().unwrap())..i - 2]
                                                .rotate_right(a.to_usize().unwrap());
                                            // i -= k as usize;

                                            i = 0;
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                i += 1;
            }

            // Refold
            if new_block.len() > 0 {
                i = new_block.len()-1;
                'test: loop {
                    if i+2 < new_block.len() {
                        if let [Instr(Push(a)), Instr(Push(b))] = &new_block[i..i + 2] {
                            if a == b {
                                new_block[i+1] = Instr(Dup);
                                continue
                            }
                        }
                    }

                    if i == 0 {
                        break 'test
                    }
                    i -= 1;
                }
            }

            self.blocks.insert(k, new_block);
        }
    }
}
