use std::io;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input line");
    input.trim_right().to_string()
}

#[derive(Debug)]
enum Operator {
    Mul,
    Sum,
    Sub,
}

#[derive(Debug)]
enum Operand {
    Value(i32),
    Variable(String)
}

#[derive(Debug)]
enum Token {
    Operator(Operator),
    Operand(Operand)
}

fn next_token(expr: &str, offset: &mut usize) -> Option<Token> {
    /*lazy_static! {
        static ref REG_NUM: Regex = Regex::new(r"^\d+").unwrap();
        static ref REG_VAR: Regex = Regex::new(r"^\w+\d*").unwrap();
    }*/
    if expr.is_empty() {
        None
    }
    else if expr.starts_with("+") {
        *offset += 1;
        Some(Token::Operator(Operator::Sum))
    }
    else if expr.starts_with("-") {
        *offset += 1;
        Some(Token::Operator(Operator::Sub))
    }
    else if expr.starts_with("*") {
        *offset += 1;
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
            *offset += number.len();
            Some(Token::Operand(Operand::Value(number.parse::<i32>().unwrap())))
        }
        else if c.is_alphabetic() {
            let mut len = 1;
            while let Some(c) = chars.next() {
                if !c.is_alphanumeric() { break; }
                len += 1;
            }
            let variable = &expr[0..len];
            *offset += variable.len();
            Some(Token::Operand(Operand::Variable(variable.to_string())))
        }
        else { None }
    }
}

fn main() {
    let expr = read_line().replace(" ", "");
    let mut offset = 0;
    while let Some(token) = next_token(&expr[offset..], &mut offset) {
        println!("{:?}", token);
    }
}
