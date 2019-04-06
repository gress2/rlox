use std::vec::Vec;
use std::iter::Peekable;
use std::str::Chars;
use token::Token;
use token::TokenType;
use token::Printable;

fn build_token(type_ : TokenType) -> Token {
    return Token { 
        type_ : type_,
        lexeme : "stuff".to_string(),
        line : 0
    };
}

fn next_matches(c: char, char_iter: &mut Peekable<Chars>) -> bool {
    match char_iter.peek() {
        Some(ch) => return *ch == c,
        None => return false
    }
}

fn consume_comment(char_iter: &mut Peekable<Chars>) -> () {
    loop {
        match char_iter.next() {
            Some(c) => {
                match c {
                    '\n' => break,
                    _ => ()
                }
            },
            None => break
        }
    }
}

fn get_string(char_iter: &mut Peekable<Chars>) -> String {
    let mut s : String = "".to_string();
    loop {
        match char_iter.next() {
            Some(c) => {
                if c == '"' {
                    return s;
                } else {
                    s.push(c);
                }
            },
            None => {
                println!("Unterminated string");
                return s;
            }
        }
    }
}

fn get_digits(c: char, char_iter: &mut Peekable<Chars>) -> f64 {
    let mut digit_str : String = c.to_string();

    while char_iter.peek().unwrap().is_digit(10) {
        digit_str.push(char_iter.next().unwrap());
    }
    let mut has_decimal = *(char_iter.peek().unwrap()) == '.';

    if has_decimal {
        digit_str.push(char_iter.next().unwrap());
        while char_iter.peek().unwrap().is_digit(10) {
            digit_str.push(char_iter.next().unwrap());
        }
    }

    match digit_str.parse::<f64>() {
        Ok(n) => return n,
        Err(e) => {
            println!("Failed to parse digit");
            return 0.0;
        }
    }
}

fn get_identifier(c: char, char_iter: &mut Peekable<Chars>) -> String {
    return "".to_string();
}

fn scan_token(mut char_iter: &mut Peekable<Chars>, src: &String) -> Token {
    println!("scan_token");
    let prev = (*char_iter).clone();
    match char_iter.next() {
        Some(c) => match c {
            '(' => return build_token(TokenType::LeftParen),
            ')' => return build_token(TokenType::RightParen),
            '{' => return build_token(TokenType::LeftBrace),
            '}' => return build_token(TokenType::RightBrace),
            ',' => return build_token(TokenType::Comma),
            '.' => return build_token(TokenType::Dot),
            '-' => return build_token(TokenType::Minus),
            '+' => return build_token(TokenType::Plus),
            ';' => return build_token(TokenType::SemiColon),
            '*' => return build_token(TokenType::Star),
            '!' => {
                if next_matches('=', &mut char_iter) {
                    return build_token(TokenType::BangEqual);
                } else {
                    return build_token(TokenType::Bang);
                }
            }, 
            '=' => {
                if next_matches('=', &mut char_iter) {
                    return build_token(TokenType::EqualEqual);
                } else {
                    return build_token(TokenType::Equal);
                }
            },
            '<' => {
                if next_matches('=', &mut char_iter) {
                    return build_token(TokenType::LessEqual);
                } else {
                    return build_token(TokenType::Less);
                }
            },
            '>' => {
                if next_matches('=', &mut char_iter) {
                    return build_token(TokenType::GreaterEqual);
                } else {
                    return build_token(TokenType::Greater);
                }
            },
            '/' => {
                if next_matches('/', &mut char_iter) {
                    consume_comment(&mut char_iter);
                    return build_token(TokenType::Null);
                } else {
                    return build_token(TokenType::Slash);
                }
    
            },
            ' ' => return build_token(TokenType::Null), 
            '\r' => return build_token(TokenType::Null),
            '\t' => return build_token(TokenType::Null),
            '"' => {
                let s : String = get_string(&mut char_iter);
                println!("{}", s);
                return build_token(TokenType::String_);
            },
            _ => {
                if c.is_digit(10) {
                    let d : f64 = get_digits(c, &mut char_iter);
                    println!("{}", d);
                    return build_token(TokenType::Number);
                } else if c.is_alphabetic() {
                    let s : String = get_identifier(c, &mut char_iter);
                    println!("{}", s);
                    return build_token(TokenType::Identifier);
                } else {
                    return build_token(TokenType::Null);
                }
            }
        },
        None => return Token {
            type_: TokenType::Eof,
            lexeme: "stuff".to_string(),
            line: 4
        }
    }
}

pub fn scan_tokens(source: String) -> Vec<Token> {
    let mut tokens : Vec<Token> = Vec::new();
    let mut done : bool = false;
    let mut cur : usize = 0;

    let src_len = source.len();
    let mut char_iter = source.chars().peekable();

    loop {
        let tkn = scan_token(&mut char_iter, &source);
        match tkn {
            Token { type_: TokenType::Eof, .. } => {
                tokens.push(tkn);    
                break;
            },
            Token { type_: TokenType::Null, .. } => (),
            _ => tokens.push(tkn)
        }
    }

    for token in tokens.iter() {
        token.print();
    }

    return tokens;
}
