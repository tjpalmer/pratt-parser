use super::tokenizer::Token;
use super::tokenizer::Symbol;
use super::tokenizer::Tokenizer;

use std::iter::Iterator;
use std::iter::Peekable;

#[derive(Debug,PartialEq)]
pub enum Expr {
    Integer(i32),
    BinaryExpr(Box<Expr>, Op, Box<Expr>)
}

#[derive(Debug,PartialEq)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn get_precedence(symbol: &Symbol) -> u8 {
    match symbol {
        &Symbol::Add | &Symbol::Subtract => 10,
        &Symbol::Multiply | &Symbol::Divide => 20,
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Expr, String> {
    let mut it = tokens.iter().peekable();
    parse_expr(&mut it, 0)
}

fn parse_expr<'a, It>(it: &mut Peekable<It>, precedence: u8) -> Result<Expr, String>
    where It: Iterator<Item=&'a Token> {

    println!("parse_expr() calling parse_prefix()");

    let mut expr = parse_prefix(it).unwrap();

    while let Some(&next_token) = it.peek() {

        println!("parse_expr() left = {:?}", expr);

        let next_precedence = match next_token {
            &Token::Operator(ref symbol) => get_precedence(symbol),
            _ => panic!("Expected operator after expr")
        };

        if precedence >= next_precedence {
            println!("parse_expr() breaking out of loop due to precedence change");
            break;
        }

        println!("parse_expr() calling parse_infix()");

        expr = parse_infix(expr, it, next_precedence).unwrap();
    }

    println!("parse_expr() returning {:?}", expr);

    Ok(expr)
}

fn parse_prefix<'a, It>(it: &mut Peekable<It>) -> Result<Expr, String>
    where It: Iterator<Item=&'a Token> {

    match it.peek() {
        Some(&t) => match t {
            &Token::Integer(n) => {
                it.next().unwrap(); // consume the token
                Ok(Expr::Integer(n))
            },
            _ => Err(format!("unexpected token: {:?}", t))
        },
        None => Err(String::from("No more tokens"))
    }
}

fn parse_infix<'a, It>(left: Expr, it: &mut Peekable<It>, precendence: u8) -> Result<Expr, String>
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

                it.next().unwrap(); // consume the token

                println!("parse_infix() parsed operator {:?} and now calling parse_expr()", op);

                let right = parse_expr(it, precendence).unwrap();

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

//#[test]
fn parse_plus_then_multiply() {


    let equation = String::from("123+456*789").tokenize();

    let ast = parse(equation);

    assert_eq!(Ok(Expr::BinaryExpr(
        Box::new(Expr::Integer(123)),
        Op::Add,
        Box::new(Expr::BinaryExpr(
                Box::new(Expr::Integer(456)),
                Op::Multiply,
                Box::new(Expr::Integer(789))
            )
        ))
    ), ast);

}

#[test]
fn parse_multiply_then_plus() {

    let equation = String::from("123*456+789").tokenize();

    let ast = parse(equation);

    assert_eq!(Ok(Expr::BinaryExpr(
        Box::new(Expr::BinaryExpr(
            Box::new(Expr::Integer(123)),
            Op::Multiply,
            Box::new(Expr::Integer(456))
        )),
        Op::Add,
        Box::new(Expr::Integer(789))
    )), ast);

}
