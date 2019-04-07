#[derive(Debug)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,

    Identifier, String_, Number,

    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Null,

    Eof
}

pub trait Printable {
    fn print(&self) -> ();
}

pub struct Token {
    pub type_: TokenType,
    pub lexeme: String,
    pub line: i8,
    pub num_: Option<f64>,
    pub str_: Option<String>
}

impl Printable for Token {
    fn print(&self) {
        println!("L{0}: {1:?} {2}", self.line, self.type_, self.lexeme);
    }
}
