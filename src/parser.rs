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

fn parse(tokens: Vec<Token>) -> Result<Expr, &'static str> {
    let mut it = tokens.iter().peekable();
    parse_prefix(&mut it);
    Err("not implemented yet")
}

fn parse_prefix<'a, It>(it: &mut Peekable<It>) -> Result<Expr, &'static str>
    where It: Iterator<Item=&'a Token> {

    match it.peek() {
        Some(&t) => match t {
            &Token::Integer(n) => Ok(Expr::Integer(n)),
            _ => Err("TBD")
        },
        None => Err("no more tokens")
    }
}
//
// fn parse_infix() -> Result<Expr, &'static str> {
//     Err("not implemented yet")
// }
