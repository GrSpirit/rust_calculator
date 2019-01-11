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
                Operator::Mul => 2,
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
    fn calc() -> i32 {

    }
}

struct TokenIterator<'a> {
    expr: &'a str,
    offset: usize
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token(&self.expr[self.offset..])
    }
}

impl<'a> TokenIterator<'a> {
    fn new(expr: &'a str) -> Self {
        TokenIterator {expr: expr, offset: 0}
    }
    fn next_token(&mut self, expr: &'a str) -> Option<Token> {
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
    eprintln!("{:?}", tokens);
    let root = build_tree(tokens.as_slice());

    eprintln!("{:?}", root);
}
