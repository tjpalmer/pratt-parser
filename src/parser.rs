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
    let it = tokens.iter().peekable();
    // parse_prefix(&it);
    Err("not implemented yet")
}

// fn parse_prefix(it: &Peekable<Token>) -> Result<Expr, &'static str> {
//     Err("not implemented yet")
// }
//
// fn parse_infix() -> Result<Expr, &'static str> {
//     Err("not implemented yet")
// }
