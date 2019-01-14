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
    Operand(Operand),
    BracketOpen,
    BracketClose
}

impl Operator {
    fn precedence(&self) -> u32 {
        match self {
            Operator::Mul | Operator::Div => 2,
            Operator::Sum | Operator::Sub => 1
        }
    }
    fn calc(&self, left: i32, right: i32) -> i32 {
        match self {
            Operator::Mul => left * right,
            Operator::Div => left / right,
            Operator::Sum => left + right,
            Operator::Sub => left - right
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
        else if expr.starts_with("(") {
            self.offset += 1;
            Some(Token::BracketOpen)
        }
        else if expr.starts_with(")") {
            self.offset += 1;
            Some(Token::BracketClose)
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
    for token in tokens.into_iter() {
        match **token {
            Token::Operand(_) => result.push(token.clone()),
            Token::Operator(ref op) => {
                while let Some(top) = stack.pop() {
                    match *top.clone() {
                        Token::Operator(ref p) => 
                            if op.precedence() > p.precedence() { 
                                stack.push(top); 
                                break; 
                            }
                            else { result.push(top) },
                        Token::BracketOpen => {
                            stack.push(top);
                            break;
                        }
                        _ => panic!("Operand is not expected in the operator stack")
                    };
                }
                stack.push(token.clone());
            },
            Token::BracketOpen => stack.push(token.clone()),
            Token::BracketClose => {
                while let Some(top) = stack.pop() {
                    match *top.clone() {
                        Token::BracketOpen => break,
                        _ => result.push(top.clone())
                    }
                }
            }
        }
    }
    while let Some(token) = stack.pop() {
        result.push(token.clone());
    }
    result
}

fn calc(tokens: &Vec<Rc<Token>>, variables: &Vec<i32>) -> i32 {
    let mut i = 0;
    let mut stack: Vec<i32> = Vec::new();
    for token in tokens.iter() {
        match **token {
            Token::Operand(ref op) => {
                let val = match op {
                    Operand::Value(x) => *x,
                    Operand::Variable(_) => { i += 1; variables[i-1] }
                };
                stack.push(val);
            },
            Token::Operator(ref op) => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(op.calc(left, right));
            },
            _ => ()
        }
    }
    stack.pop().unwrap()
}

fn get_variants(vars: &[i32]) -> Vec<Vec<i32>> {
    if vars.len() == 0 {
        return vec![];
    }
    let variants = get_variants(&vars[1..]);
    (1..=vars[0]).map(|i| {
        variants
            .iter()
            .map(|vv| vec![vec![i], vv.clone()].into_iter().flatten().collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()
    })
    .flatten()
    .collect::<Vec<_>>()
}

// Start point
fn main() {
    let expr = read_line().replace(" ", "");
    let tokens: Vec<Rc<Token>> = TokenIterator::new(expr.as_str()).map(|x| Rc::new(x)).collect();
    let mut variables = tokens.iter()
        //.filter_map(|x| match **x { Token::Operand(ref op) => match op { Operand::Variable(v) => Some(Variable::new(v.clone())), _=> None}, _=> None})
        .filter_map(|x| match **x { Token::Operand(ref op) => match op { Operand::Variable(v) => Some(v[1..].parse::<i32>().unwrap()), _=> None}, _=> None})
        .collect::<Vec<_>>();
    eprintln!("{:?}", tokens);
    let tokens = infix_to_postfix(&tokens);
    eprintln!("{:?}", tokens);
    eprintln!("{:?}", variables);

    //let mut last = false;
    let variants = get_variants(&variables[..]);
    eprintln!("{:?}", variants);
    /*while !last {
        last = true;
        for v in variables.iter_mut() {
            (*v).inc();
            last = last && ((*v).val == (*v).end);
        }
        println!("{}", calc(&tokens, &variables.iter().map(|v| v.val).collect::<Vec<_>>()));
    }*/
    for v in &variants {
        println!("{}", calc(&tokens, v));
    }
}
