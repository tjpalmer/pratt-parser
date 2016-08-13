use std::iter::Iterator;
use std::iter::Peekable;
use std::str::Chars;



#[derive(Debug, PartialEq)]
enum Symbol {
    Plus,
    Minus,
    Mult,
    Divide
}

#[derive(Debug, PartialEq)]
enum Token {
    Integer(i32),
    Operator(Symbol)
}


trait Tokenizer {
    fn tokenize(&self) -> Vec<Token>;
}

fn my_take_while<F>(it: &mut Peekable<Chars>, x: F) -> Vec<char>
    where F : Fn(char) -> bool {

    let mut v: Vec<char> = vec![];

    while let Some(&ch) = it.peek() {
        if x(ch) {
            it.next().unwrap();
            v.push(ch);
        } else {
            break;
        }
    }

    v
}

impl Tokenizer for String {

    fn tokenize(&self) -> Vec<Token> {

        // get an iterator over the chars
        let mut it = self.chars().peekable();

        let mut tokens: Vec<Token> = vec![];

        loop {
            // it.peek() returns Option<&Char>
            match it.peek() {
                // cannot use 'ref ch' but '&ch' works ... why?
                Some(&ch) => match ch {
                    '0' ... '9' => {
                        let num: String = my_take_while(&mut it, |a| a.is_numeric())
                            .into_iter()
                            .collect();
                        tokens.push(Token::Integer(num.parse::<i32>().unwrap()));

                    },
                    '+' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::Plus))
                    },
                    _ => panic!("invalid char")
                },
                None => break
            }
        }

        tokens
    }

}

#[test]
fn it_works() {

    let equation = String::from("123+456").tokenize();
    assert_eq!(vec![
        Token::Integer(123),
        Token::Operator(Symbol::Plus),
        Token::Integer(456)
        ], equation);

}
