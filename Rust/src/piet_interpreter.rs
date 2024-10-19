#[derive(Debug, Copy, Clone)]
pub enum CMD {
    Push(isize),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Not,
    Greater,
    Dup,
    Roll,
    InN,
    InC,
    OutN,
    OutC,
}

impl CMD {
    pub fn interpret(
        self,
        stack: &mut Vec<isize>,
        input: &mut std::iter::Peekable<std::io::Bytes<std::io::Stdin>>,
    ) {
        match self {
            CMD::Push(v) => stack.push(v),
            CMD::Pop => {
                stack.pop();
            }
            CMD::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b + a);
            }
            CMD::Sub => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            CMD::Mul => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b * a);
            }
            CMD::Div => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b / a);
            }
            CMD::Mod => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b.rem_euclid(a));
            }
            CMD::Not => {
                let a = stack.pop().unwrap();
                stack.push(if a == 0 { 1 } else { 0 });
            }
            CMD::Greater => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(if b > a { 1 } else { 0 });
            }
            CMD::Dup => {
                let a = stack.pop().unwrap();
                stack.push(a);
                stack.push(a);
            }
            CMD::Roll => {
                let mut a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                a = a.rem_euclid(b);

                let a = a as usize;
                let b = b as usize;

                if a != 0 {
                    let s = stack.len();
                    stack[s - b..].rotate_right(a);
                }
            }
            CMD::InN => {
                let mut char_vec: Vec<char> = Vec::new();

                while let Some(Ok(c)) = input.peek() {
                    if let Some(a) = char::from_u32(*c as u32) {
                        if a.is_digit(10) {
                            char_vec.push(a);
                            input.next();
                            continue;
                        }
                    }

                    break;
                }

                if input.size_hint().0 == 0 {
                    stack.push(-1isize);
                } else if char_vec.len() == 0 {
                    // stack.push(-1isize);
                } else {
                    stack.push(
                        char_vec
                            .iter()
                            .cloned()
                            .collect::<String>()
                            .parse::<isize>()
                            .unwrap(),
                    );
                }
            }
            CMD::InC => {
                if let Some(Ok(c)) = input.next() {
                    stack.push(c as isize)
                } else {
                    stack.push(-1isize)
                }
            }
            CMD::OutN => {
                let a = stack.pop().unwrap();
                print!("{}", a);
            }
            CMD::OutC => {
                let a = stack.pop().unwrap();
                let c = char::from_u32(a as u32).unwrap();
                print!("{}", c);
            }
        }
    }
}
