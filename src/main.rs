use std::char;
use std::iter;
use std::str::FromStr;
use std::vec::Vec;

enum Token {
    Number(i64),
    Plus,
    Dash,
    Star,
    Slash,
    LeftParen,
    RightParen,
    EOF,
}
fn main() {
    fn tokenizer(input: String) -> Result<Vec<Token>, SyntaxError> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut iter = input.chars().peekable();

        while let Some(ch) = iter.next() {
            match ch {
                ch if ch.is_whitespace() => continue,
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Dash),
                '*' => tokens.push(Token::Star),
                '1'..='9' => {
                    let n: i64 = iter::once(ch)
                        .chain(iter.by_ref().filter(|s| s.is_ascii_digit()))
                        .collect::<String>()
                        .parse()
                        .unwrap();

                    tokens.push(Token::Number(n));
                }
                _ => {
                    panic!("unrecognized char");
                }
            }
        }
        tokens.push(Token::EOF);
        Ok(tokens)
    }
}

#[derive(Debug)]
struct SyntaxError {
    message: String,
}

impl SyntaxError {
    fn new(message: String) -> Self {
        SyntaxError { message }
    }
}
