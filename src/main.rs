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

// Expression operands
#[derive(Debug, Clone)]
enum Operand {
    Value(i32),
    Variable(String)
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
    fn calc(&self) -> i32 {
        match self.value {
            Token::Operand(ref op) => match op {
                Operand::Value(x) => *x,
                Operand::Variable(_) => {
                    unsafe {
                        VARS_I += 1; 
                        VARS[VARS_I - 1]
                    }
                }
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
                Some(Token::Operand(Operand::Variable(String::from(variable))))
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

static mut VARS: [i32;10] = [0; 10];
static mut VARS_I: usize = 0;

#[derive(Debug)]
struct Variable {
    name: String,
    val: i32,
    start: i32,
    end: i32
}

impl Variable {
    fn new(name: String) -> Self {
        let end = name[1..].parse::<i32>().unwrap();
        Variable { name: name, val: 0, start: 1, end: end }
    }
    fn inc(&mut self) -> i32 {
        self.val = if self.val == self.end { self.start } else { self.val + 1 };
        self.val
    }
}
// Start point
fn main() {
    let expr = read_line().replace(" ", "");
    let tokens: Vec<Token> = TokenIterator::new(expr.as_str()).collect();
    let mut variables = tokens.iter()
        .filter_map(|x| match x { Token::Operand(op) => match op { Operand::Variable(v) => Some(Variable::new(v.clone())), _=> None}, _=> None})
        .collect::<Vec<_>>();
    eprintln!("{:?}", tokens);
    let root = build_tree(tokens.as_slice()).unwrap();
    eprintln!("{:?}", root);
    let mut last = false;
    while !last {
        last = true;
        unsafe {
            VARS_I = 0;
            for v in variables.iter_mut() {
                VARS[VARS_I] = (*v).inc();
                VARS_I += 1;
                last = last && ((*v).val == (*v).end);
            }
            VARS_I = 0;
        }
        println!("{}", root.calc());
    }
}
