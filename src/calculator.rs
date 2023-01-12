#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char),
}

pub struct Calculator {}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchedParens,
}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens: Vec<char> = Vec::new();
        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        *n = *n * 10 + (c as u32 - 48);
                    }
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Number(digit));
                    }
                },
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                ' ' => {}
                '\n' => {}
                '\r' => {}
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push('(');
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    match parens.pop() {
                        Some('(') => {}
                        _ => return Err(Error::MismatchedParens),
                    }
                }
                _ => return Err(Error::BadToken(c)),
            }
        }

        if parens.len() > 0 {
            return Err(Error::MismatchedParens);
        }

        Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty() && stack[stack.len() - 1] >= token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                }
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                }
                _ => {}
            }
        }

        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }

        queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(n) => stack.push(n as f32),
                Token::Op(op) => {
                    let right: f32 = stack.pop().unwrap();
                    let left: f32 = stack.pop().unwrap();
                    match op {
                        Operator::Add => stack.push(left + right),
                        Operator::Sub => stack.push(left - right),
                        Operator::Mul => stack.push(left * right),
                        Operator::Div => stack.push(left / right),
                    }
                }
                _ => {}
            }
        }

        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }
}
