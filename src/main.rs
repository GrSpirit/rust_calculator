use std::rc::Rc;
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

impl Operator {
    fn precedence(&self) -> u32 {
        match self {
            Operator::Mul | Operator::Div => 2,
            Operator::Sum | Operator::Sub => 1
        }
    }
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

fn infix_to_postfix(tokens: &Vec<Rc<Token>>) -> Vec<Rc<Token>> {
    let mut result: Vec<Rc<Token>> = Vec::with_capacity(tokens.len());
    let mut stack: Vec<Rc<Token>> = Vec::new();
    for token in tokens.iter() {
        match **token {
            Token::Operand(_) => result.push(token.clone()),
            Token::Operator(ref op) => {
                while let Some(top) = stack.pop() {
                    let push_back: bool = match *top {
                        Token::Operator(ref p) => op.precedence() < p.precedence(),
                        _ => false
                    };
                    if push_back {
                        stack.push(top);
                    }
                }
                stack.push(token.clone());

                /*match stack.last().map(|v| v.clone()) {
                    Some(top) => 
                    None => stack.push(token.clone())

                }*/
            }
        }
    }
    result
}

// Start point
fn main() {
    let expr = read_line().replace(" ", "");
    let tokens: Vec<Rc<Token>> = TokenIterator::new(expr.as_str()).map(|x| Rc::new(x)).collect();
    let mut variables = tokens.iter()
        .filter_map(|x| match **x { Token::Operand(ref op) => match op { Operand::Variable(v) => Some(Variable::new(v.clone())), _=> None}, _=> None})
        .collect::<Vec<_>>();
    eprintln!("{:?}", tokens);
    /*let root = build_tree(tokens.as_slice()).unwrap();
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
    }*/
}
