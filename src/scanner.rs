use std::vec::Vec;
use std::iter::Peekable;
use std::str::Chars;
use token::Token;
use token::TokenType;
use token::Printable;

fn build_token(type_ : TokenType, line : &i8) -> Token {
    return Token { 
        type_ : type_,
        lexeme : "".to_string(),
        line : *line,
        num_ : None,
        str_ : None
    };
}

fn build_num_token(num : f64, line : &i8) -> Token {
    return Token {
        type_ : TokenType::Number,
        lexeme : "".to_string(),
        line : *line,
        num_ : Some(num),
        str_ : None
    };
}

fn build_str_token(s : String, line : &i8) -> Token {
    return Token {
        type_ : TokenType::String_,
        lexeme : "".to_string(),
        line : *line,
        num_ : None,
        str_ : Some(s)
    };
}

fn next_matches(c: char, char_iter: &mut Peekable<Chars>) -> bool {
    match char_iter.peek() {
        Some(ch) => return *ch == c,
        None => return false
    }
}

fn consume_comment(char_iter: &mut Peekable<Chars>, line: &mut i8) -> () {
    loop {
        match char_iter.next() {
            Some(c) => {
                match c {
                    '\n' => {
                        *line += 1;
                        break;
                    },
                    _ => ()
                }
            },
            None => break
        }
    }
}

fn get_string(char_iter: &mut Peekable<Chars>, line: &mut i8) -> String {
    let mut s : String = "".to_string();
    loop {
        match char_iter.next() {
            Some(c) => {
                match c {
                    '\n' => {
                        *line += 1;
                        s.push(c);
                    },
                    '"' => return s,
                    _ => s.push(c)
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
    let has_decimal = *(char_iter.peek().unwrap()) == '.';

    if has_decimal {
        digit_str.push(char_iter.next().unwrap());
        while char_iter.peek().unwrap().is_digit(10) {
            digit_str.push(char_iter.next().unwrap());
        }
    }

    match digit_str.parse::<f64>() {
        Ok(n) => return n,
        Err(_) => {
            println!("Failed to parse number");
            return 0.0;
        }
    }
}

fn get_identifier(c: char, char_iter: &mut Peekable<Chars>) -> String {
    let mut id : String = c.to_string();

    while char_iter.peek().unwrap().is_alphanumeric() {
        id.push(char_iter.next().unwrap());
    }

    return id;
}

fn scan_token(mut char_iter: &mut Peekable<Chars>, mut line: &mut i8) -> Token {
    match char_iter.next() {
        Some(c) => match c {
            '(' => return build_token(TokenType::LeftParen, &line),
            ')' => return build_token(TokenType::RightParen, &line),
            '{' => return build_token(TokenType::LeftBrace, &line),
            '}' => return build_token(TokenType::RightBrace, &line),
            ',' => return build_token(TokenType::Comma, &line),
            '.' => return build_token(TokenType::Dot, &line),
            '-' => return build_token(TokenType::Minus, &line),
            '+' => return build_token(TokenType::Plus, &line),
            ';' => return build_token(TokenType::SemiColon, &line),
            '*' => return build_token(TokenType::Star, &line),
            '!' => {
                if next_matches('=', &mut char_iter) {
                    return build_token(TokenType::BangEqual, &line);
                } else {
                    return build_token(TokenType::Bang, &line);
                }
            }, 
            '=' => {
                if next_matches('=', &mut char_iter) {
                    return build_token(TokenType::EqualEqual, &line);
                } else {
                    return build_token(TokenType::Equal, &line);
                }
            },
            '<' => {
                if next_matches('=', &mut char_iter) {
                    return build_token(TokenType::LessEqual, &line);
                } else {
                    return build_token(TokenType::Less, &line);
                }
            },
            '>' => {
                if next_matches('=', &mut char_iter) {
                    return build_token(TokenType::GreaterEqual, &line);
                } else {
                    return build_token(TokenType::Greater, &line);
                }
            },
            '/' => {
                if next_matches('/', &mut char_iter) {
                    consume_comment(&mut char_iter, &mut line);
                    return build_token(TokenType::Null, &line);
                } else {
                    return build_token(TokenType::Slash, &line);
                }
            },
            ' ' => return build_token(TokenType::Null, &line), 
            '\r' => return build_token(TokenType::Null, &line),
            '\t' => return build_token(TokenType::Null, &line),
            '\n' => {
                *line += 1;   
                return build_token(TokenType::Null, &line);
            },
            '"' => {
                let s : String = get_string(&mut char_iter, &mut line);
                return build_str_token(s, &line);
            },
            _ => {
                if c.is_digit(10) {
                    let d : f64 = get_digits(c, &mut char_iter);
                    return build_num_token(d, &line);
                } else if c.is_alphabetic() {
                    let s : String = get_identifier(c, &mut char_iter);

                    match s.as_ref() {
                        "and" => return build_token(TokenType::And, &line),
                        "class" => return build_token(TokenType::Class, &line),
                        "else" => return build_token(TokenType::Else, &line),
                        "false" => return build_token(TokenType::False, &line),
                        "fun" => return build_token(TokenType::Fun, &line),
                        "for" => return build_token(TokenType::For, &line),
                        "if" => return build_token(TokenType::If, &line),
                        "nil" => return build_token(TokenType::Nil, &line),
                        "or" => return build_token(TokenType::Or, &line),
                        "print" => return build_token(TokenType::Print, &line),
                        "return" => return build_token(TokenType::Return, &line),
                        "super" => return build_token(TokenType::Super, &line),
                        "this" => return build_token(TokenType::This, &line),
                        "true" => return build_token(TokenType::True, &line),
                        "var" => return build_token(TokenType::Var, &line),
                        "while" => return build_token(TokenType::While, &line),
                        _ => return build_token(TokenType::Identifier, &line)
                    }
                } else {
                    return build_token(TokenType::Null, &line);
                }
            }
        },
        None => return build_token(TokenType::Eof, &line)
    }
}

pub fn scan_tokens(source: String) -> Vec<Token> {
    let mut tokens : Vec<Token> = Vec::new();

    let src_len = source.len();
    let mut char_iter = source.chars().peekable();

    let mut line : i8 = 1;

    loop {
        let tkn = scan_token(&mut char_iter, &mut line);
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
