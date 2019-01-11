use std::io;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input line");
    input.trim_right().to_string()
}

// Arithmetic operators
#[derive(Debug, Clone)]
enum Operator {
    Mul,
    Sum,
    Sub,
    Div
}

#[derive(Debug, Clone)]
struct Var {
    name: String,
    val: i32
}

impl Var {
    fn new(name: &str) -> Self {
        Var { name: String::from(name), val: 0 }
    }
}
// Expression operands
#[derive(Debug, Clone)]
enum Operand {
    Value(i32),
    Variable(Var)
}

// Token
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

// Expression Tree
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
    fn calc(&self, vars: &[i32], vi: &mut usize) -> i32 {
        match self.value {
            Token::Operand(ref op) => match op {
                Operand::Value(x) => *x,
                Operand::Variable(_) => {
                    *vi += 1;
                    vars[*vi - 1]
                }
            },
            Token::Operator(ref op) => match op {
                Operator::Mul => {
                    let left = self.left.as_ref().unwrap();
                    let right = self.right.as_ref().unwrap();
                    left.calc(vars, vi) * right.calc(vars, vi)
                },
                Operator::Div => {
                    let left = self.left.as_ref().unwrap();
                    let right = self.right.as_ref().unwrap();
                    left.calc(vars, vi) / right.calc(vars, vi)
                },
                Operator::Sum => {
                    let left = self.left.as_ref().unwrap();
                    let right = self.right.as_ref().unwrap();
                    left.calc(vars, vi) + right.calc(vars, vi)
                },
                Operator::Sub => {
                    let left = self.left.as_ref().unwrap();
                    let right = self.right.as_ref().unwrap();
                    left.calc(vars, vi) - right.calc(vars, vi)
                },
            }
        }

    }
    /*fn assign_variables(&mut self, vars: &[i32], i: &mut usize) {
        match self.value {
            Token::Operand(ref mut op) => {
                if let Operand::Variable(ref mut v) = op {
                    v.val = vars[*i];
                    *i += 1;
                }
            },
            Token::Operator(_) => {
                let mut left = self.left.as_ref().unwrap();
                left.assign_variables(vars, &mut i);
                let mut right = self.right.as_ref().unwrap();
                right.assign_variables(vars, &mut i);
            }
        }
    }*/
}

// Token iterator to parse expression string
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
                Some(Token::Operand(Operand::Variable(Var::new(variable))))
            }
            else { None }
        }
    }
}

// Build expression string
fn build_tree(expr: &[Token]) -> Option<Box<Node>> {
    if let Some((i, t)) = expr.iter()
        .enumerate()
        .min_by_key(|(_, t)| t.priority()) {
            Some(Box::new(Node {value: t.clone(), left: build_tree(&expr[..i]), right: build_tree(&expr[i+1..])}))
        }
    else { None }
}

// Start point
fn main() {
    let expr = read_line().replace(" ", "");
    let tokens: Vec<Token> = TokenIterator::new(expr.as_str()).collect();
    let variables = tokens.iter().filter(|x| match x { Token::Operand(op) => match op { Operand::Variable(_) => true, _=> false}, _=> false});
    eprintln!("{:?}", tokens);
    let mut root = build_tree(tokens.as_slice());
    eprintln!("{:?}", root);
    let mut vi = 0;
    println!("{}", root.unwrap().calc(&[0,1], &mut vi));
}
