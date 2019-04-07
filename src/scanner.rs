use std::vec::Vec;
use std::iter::Peekable;
use std::str::Chars;
use token::Token;
use token::TokenType;
use token::Printable;

fn build_token(type_ : TokenType, line : &i8, start_iter : &Peekable<Chars>, end_iter : &Peekable<Chars>) -> Token {

    /*let lexeme : String =

    let mut tkn : Token = {
        type_ : type_,

    }

    match type_ {
        TokenType::Number => {
            return Token {}
        }
    }*/



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
    let start_iter : Peekable<Chars> = char_iter.clone();

    let tokenize = | ty: TokenType | -> Token {
        build_token(ty, &line, &start_iter, &char_iter)
    };

    match char_iter.next() {
        Some(c) => match c {
            '(' => return tokenize(TokenType::LeftParen),
            ')' => return tokenize(TokenType::RightParen),
            '{' => return tokenize(TokenType::LeftBrace),
            '}' => return tokenize(TokenType::RightBrace),
            ',' => return tokenize(TokenType::Comma),
            '.' => return tokenize(TokenType::Dot),
            '-' => return tokenize(TokenType::Minus),
            '+' => return tokenize(TokenType::Plus),
            ';' => return tokenize(TokenType::SemiColon),
            '*' => return tokenize(TokenType::Star),
            '!' => {
                if next_matches('=', &mut char_iter) {
                    return tokenize(TokenType::BangEqual);
                } else {
                    return tokenize(TokenType::Bang);
                }
            },
            '=' => {
                if next_matches('=', &mut char_iter) {
                    return tokenize(TokenType::EqualEqual);
                } else {
                    return tokenize(TokenType::Equal);
                }
            },
            '<' => {
                if next_matches('=', &mut char_iter) {
                    return tokenize(TokenType::LessEqual);
                } else {
                    return tokenize(TokenType::Less);
                }
            },
            '>' => {
                if next_matches('=', &mut char_iter) {
                    return tokenize(TokenType::GreaterEqual);
                } else {
                    return tokenize(TokenType::Greater);
                }
            },
            '/' => {
                if next_matches('/', &mut char_iter) {
                    consume_comment(&mut char_iter, &mut line);
                    return tokenize(TokenType::Null);
                } else {
                    return tokenize(TokenType::Slash);
                }
            },
            ' ' => return tokenize(TokenType::Null),
            '\r' => return tokenize(TokenType::Null),
            '\t' => return tokenize(TokenType::Null),
            '\n' => {
                *line += 1;
                return tokenize(TokenType::Null);
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
                        "and" => return tokenize(TokenType::And),
                        "class" => return tokenize(TokenType::Class),
                        "else" => return tokenize(TokenType::Else),
                        "false" => return tokenize(TokenType::False),
                        "fun" => return tokenize(TokenType::Fun),
                        "for" => return tokenize(TokenType::For),
                        "if" => return tokenize(TokenType::If),
                        "nil" => return tokenize(TokenType::Nil),
                        "or" => return tokenize(TokenType::Or),
                        "print" => return tokenize(TokenType::Print),
                        "return" => return tokenize(TokenType::Return),
                        "super" => return tokenize(TokenType::Super),
                        "this" => return tokenize(TokenType::This),
                        "true" => return tokenize(TokenType::True),
                        "var" => return tokenize(TokenType::Var),
                        "while" => return tokenize(TokenType::While),
                        _ => return tokenize(TokenType::Identifier)
                    }
                } else {
                    return tokenize(TokenType::Null);
                }
            }
        },
        None => return tokenize(TokenType::Eof)
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
