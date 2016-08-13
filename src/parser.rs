use super::tokenizer::Token;
use super::tokenizer::Symbol;

use std::iter::Iterator;
use std::iter::Peekable;

#[derive(Debug,PartialEq)]
enum Expr {
    Integer(i32),
    BinaryExpr(Box<Expr>, Op, Box<Expr>)
}

#[derive(Debug,PartialEq)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut it = tokens.iter().peekable();
    parse_expr(&mut it, 0)
}

fn parse_expr<'a, It>(it: &mut Peekable<It>, precendence: u8) -> Result<Expr, String>
    where It: Iterator<Item=&'a Token> {

    parse_prefix(it)
}

fn parse_prefix<'a, It>(it: &mut Peekable<It>) -> Result<Expr, String>
    where It: Iterator<Item=&'a Token> {

    match it.peek() {
        Some(&t) => match t {
            &Token::Integer(n) => Ok(Expr::Integer(n)),
            _ => Err(format!("unexpected token: {:?}", t))
        },
        None => Err(String::from("No more tokens"))
    }
}

fn parse_infix<'a, It>(left: Expr, it: &mut Peekable<It>) -> Result<Expr, String>
    where It: Iterator<Item=&'a Token> {

    match it.peek() {
        Some(&t) => match t {
            &Token::Operator(ref s) => {

                let op = match s {
                    &Symbol::Add => Op::Add,
                    &Symbol::Subtract => Op::Subtract,
                    &Symbol::Multiply => Op::Multiply,
                    &Symbol::Divide => Op::Divide,
                };

                let right = parse_expr(it, 0).unwrap();

                Ok(Expr::BinaryExpr(
                    Box::new(left),
                    op,
                    Box::new(right)))
            },
            _ => Err(format!("Unexpected token {:?}", t))
        },
        None => Err(String::from("No more tokens"))
    }

}
