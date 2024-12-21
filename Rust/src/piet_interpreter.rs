use num::BigInt;
use num::ToPrimitive;
use num::traits::Euclid;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CMD {
    Nop,

    Push(BigInt),
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
use CMD::*;

impl CMD {
    pub fn cmd_str(self) -> String {
        String::from(
            match self {
                Nop => "",

                Push(_) => "push",
                Pop => "pop",
                Add => "add",
                Sub => "sub",
                Mul => "mul",
                Div => "div",
                Mod => "mod",
                Not => "not",
                Greater => "greater",
                Dup => "dup",
                Roll => "roll",
                InN => "inN",
                InC => "inC",
                OutN => "outN",
                OutC => "outC",

                Pointer => panic!(),
                Switch => panic!(),
            }
        )
    }

    pub fn interpret_result<I: std::io::Read, O: std::io::Write>(
        self,
        stack: &mut Vec<BigInt>,
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
                if b == 0.into() { return None }
                stack.push(b / a);
            }
            CMD::Mod => {
                if stack.len() < 2 { return None }
                let a = stack.pop()?;
                let b = stack.pop()?;
                if a == 0.into() { return None }
                stack.push(b.rem_euclid(&a));
            }
            CMD::Not => {
                if stack.len() < 1 { return None }
                let a = stack.pop()?;
                stack.push(if a == 0.into() { 1.into() } else { 0.into() });
            }
            CMD::Greater => {
                if stack.len() < 2 { return None }
                let a = stack.pop()?;
                let b = stack.pop()?;

                stack.push(if b > a { 1.into() } else { 0.into() });
            }
            CMD::Dup => {
                let a = stack.pop()?;
                stack.push(a.clone());
                stack.push(a);
            }
            CMD::Roll => {
                if stack.len() < 2 || stack[stack.len()-2] < 0.into() { return None }

                let mut a = stack.pop()?;
                let b = stack.pop()?;

                // let b = stack.pop()?;
                // if b == 0 { return None }
                a = a.rem_euclid(&b);

                // let a = a as usize;
                let b = b.to_usize().unwrap();

                if a != 0.into() {
                    let s = stack.len().clone();
                    if a > 0.into() {
                        stack[s - b..s].rotate_right(a.to_usize().unwrap());
                    } else {
                        stack[s - b..s].rotate_left((-a).to_usize().unwrap());
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

                if char_vec.len() == 0 {
                    stack.push((-1isize).into());
                } else {
                    stack.push(
                        char_vec
                            .iter()
                            .cloned()
                            .collect::<String>()
                            .parse::<BigInt>()
                            .unwrap(),
                    );
                }
            }
            CMD::InC => {
                let input = input.as_mut().unwrap();
                if let Some(Ok(c)) = input.next() {
                    stack.push(c.into())
                } else {
                    stack.push((-1).into())
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
                let a = stack.pop()?.to_u32().unwrap();
                let c = char::from_u32(a).unwrap();
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
        stack: &mut Vec<BigInt>,
        input: &mut Option<std::iter::Peekable<std::io::Bytes<I>>>,
        output: &mut Option<O>,
    ) {
        self.interpret_result(stack, input, output);
    }
}
