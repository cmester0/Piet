#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CMD {
    Nop,

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

    Pointer,
    Switch,
}

impl CMD {
    pub fn interpret_result<I: std::io::Read, O: std::io::Write>(
        self,
        stack: &mut Vec<isize>,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) -> Option<()> {
        match self {
            CMD::Nop => (),
            CMD::Push(v) => stack.push(v),
            CMD::Pop => {
                if stack.len() < 1 { return None }
                stack.pop();
            }
            CMD::Add => {
                if stack.len() < 2 { return None }
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b + a);
            }
            CMD::Sub => {
                if stack.len() < 2 { return None }
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b - a);
            }
            CMD::Mul => {
                if stack.len() < 2 { return None }
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b * a);
            }
            CMD::Div => {
                if stack.len() < 2 { return None }
                let a = stack.pop()?;
                let b = stack.pop()?;
                if b == 0 { return None }
                stack.push(b / a);
            }
            CMD::Mod => {
                if stack.len() < 2 { return None }
                let a = stack.pop()?;
                let b = stack.pop()?;
                if b == 0 { return None }
                stack.push(b.rem_euclid(a));
            }
            CMD::Not => {
                if stack.len() < 1 { return None }
                let a = stack.pop()?;
                stack.push(if a == 0 { 1 } else { 0 });
            }
            CMD::Greater => {
                if stack.len() < 2 { return None }
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(if b > a { 1 } else { 0 });
            }
            CMD::Dup => {
                let a = stack.pop()?;
                stack.push(a);
                stack.push(a);
            }
            CMD::Roll => {
                if stack.len() < 2 || stack[stack.len()-2] < 0 { return None }
                let mut a = stack.pop()?;
                let b = stack.pop()?;
                // if b == 0 { return None }
                a = a.rem_euclid(b);

                // let a = a as usize;
                let b = b as usize;

                if a != 0 {
                    let s = stack.len().clone();
                    if a > 0 {
                        stack[s - b..s].rotate_right(a as usize);
                    } else {
                        stack[s - b..s].rotate_left(-a as usize);
                    }
                }
            }
            CMD::InN => {
                let mut char_vec: Vec<char> = Vec::new();

                let input = input.as_mut().unwrap();
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

                if input.size_hint().0 == 0 && char_vec.len() == 0 {
                    stack.push(-1isize);
                // } else if char_vec.len() == 0 {
                //     // stack.push(-1isize);
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
                let input = input.as_mut().unwrap();
                if let Some(Ok(c)) = input.next() {
                    stack.push(c as isize)
                } else {
                    stack.push(-1isize)
                }
            }
            CMD::OutN => {
                if stack.len() < 1 { return None }
                let a = stack.pop()?;
                let output = output.as_mut().unwrap();
                write!(output,"{}", a).unwrap();
                output.flush().unwrap();
            }
            CMD::OutC => {
                if stack.len() < 1 { return None }
                let a = stack.pop()?;
                let c = char::from_u32(a as u32).unwrap();
                let output = output.as_mut().unwrap();
                write!(output,"{}", c).unwrap();
                output.flush().unwrap();
            }
            CMD::Pointer => {
                panic!("Cannot interpret pointer!")
            }
            CMD::Switch => {
                panic!("Cannot interpret switch!")
            }
        }

        Some(())
    }

    pub fn interpret<I: std::io::Read, O: std::io::Write>(
        self,
        stack: &mut Vec<isize>,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) {
        self.interpret_result(stack, input, output);
    }
}
