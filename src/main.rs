use std::io;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input line");
    input.trim_right().to_string()
}

#[derive(Debug, Clone)]
enum Operator {
    Mul,
    Sum,
    Sub,
    Div
}

#[derive(Debug, Clone)]
enum Operand {
    Value(i32),
    Variable(String)
}

#[derive(Debug, Clone)]
enum Token {
    Operator(Operator),
    Operand(Operand)
}

impl Token {
    fn priority(&self) -> u32 {
        match self {
            Token::Operand(_) => 10,
            Token::Operator(op) => match op {
                Operator::Mul | Operator::Div => 2,
                Operator::Sum | Operator::Sub => 1
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    value: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(t: Token) -> Self {
        Node { value: t, left: None, right: None }
    }
    fn calc(&self) -> i32 {
        let val = &self.value.clone();
        match val {
            Token::Operand(ref op) => match op {
                Operand::Value(x) => *x,
                Operand::Variable(_) => panic!("Not implemented")
            },
            Token::Operator(ref op) => match op {
                Operator::Mul => {
                    let left = self.left.as_ref().unwrap();
                    let right = self.right.as_ref().unwrap();
                    left.calc() * right.calc()
                },
                Operator::Div => {
                    let left = self.left.as_ref().unwrap();
                    let right = self.right.as_ref().unwrap();
                    left.calc() / right.calc()
                },
                Operator::Sum => {
                    let left = self.left.as_ref().unwrap();
                    let right = self.right.as_ref().unwrap();
                    left.calc() + right.calc()
                },
                Operator::Sub => {
                    let left = self.left.as_ref().unwrap();
                    let right = self.right.as_ref().unwrap();
                    left.calc() - right.calc()
                },
            }
        }

    }
}

struct TokenIterator<'a> {
    expr: &'a str,
    offset: usize
}

impl<'a> TokenIterator<'a> {
    fn new(expr: &'a str) -> Self {
        TokenIterator {expr: expr, offset: 0}
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let expr = &self.expr[self.offset..];
        /*lazy_static! {
          static ref REG_NUM: Regex = Regex::new(r"^\d+").unwrap();
          static ref REG_VAR: Regex = Regex::new(r"^\w+\d*").unwrap();
          }*/
        if expr.is_empty() {
            None
        }
        else if expr.starts_with("+") {
            self.offset += 1;
            Some(Token::Operator(Operator::Sum))
        }
        else if expr.starts_with("-") {
            self.offset += 1;
            Some(Token::Operator(Operator::Sub))
        }
        else if expr.starts_with("*") {
            self.offset += 1;
            Some(Token::Operator(Operator::Mul))
        }
        else if expr.starts_with("/") {
            self.offset += 1;
            Some(Token::Operator(Operator::Div))
        }
        /*else if let Some(cap) = REG_NUM.captures(expr) {
          let number = cap.get(0).unwrap().as_str();
         *offset += number.len();
         Some(Token::Operand(Operand::Value(number.parse::<i32>().unwrap())))
         }
         else if let Some(cap) = REG_VAR.captures(expr) {
         let variable = cap.get(0).unwrap().as_str();
         *offset += variable.len();
         Some(Token::Operand(Operand::Variable(String::from(variable))))
         }*/
        else {
            let mut chars = expr.chars();
            let c = chars.next().unwrap();
            if c.is_digit(10) {
                let mut len = 1;
                while let Some(c) = chars.next() {
                    if !c.is_digit(10) { break; }
                    len += 1;
                }
                let number = &expr[0..len];
                self.offset += number.len();
                Some(Token::Operand(Operand::Value(number.parse::<i32>().unwrap())))
            }
            else if c.is_alphabetic() {
                let mut len = 1;
                while let Some(c) = chars.next() {
                    if !c.is_alphanumeric() { break; }
                    len += 1;
                }
                let variable = &expr[0..len];
                self.offset += variable.len();
                Some(Token::Operand(Operand::Variable(variable.to_string())))
            }
            else { None }
        }
    }
}

fn build_tree(expr: &[Token]) -> Option<Box<Node>> {
    if let Some((i, t)) = expr.iter()
        .enumerate()
        .min_by_key(|(_, t)| t.priority()) {
            Some(Box::new(Node {value: t.clone(), left: build_tree(&expr[..i]), right: build_tree(&expr[i+1..])}))
        }
    else { None }
}

fn main() {
    let expr = read_line().replace(" ", "");
    let tokens: Vec<Token> = TokenIterator::new(expr.as_str()).collect();
    let variables = tokens.iter().filter(|x| match x { Token::Operand(op) => match op { Operand::Variable(_) => true, _=> false}, _=> false});
    eprintln!("{:?}", tokens);
    let root = build_tree(tokens.as_slice());
    eprintln!("{:?}", root);
    println!("{}", root.unwrap().calc());
}
