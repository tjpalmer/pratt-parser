use super::tokenizer::Token;
use super::tokenizer::Symbol;

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

fn parse(tokens: Vec<super::tokenizer::Token>) -> Expr {
    panic!("not implemented yet");
}
