#[derive(PartialEq)]
pub enum Token {
    LBRAC,   // {
    RBRAC,   // }
    LSQBRAC, // [
    RSQBRAC, // ]
    LPAREN,  // (
    RPAREN,  // )
    COLON,   // :
    COMMA,   // ,
    INTCONST(i32),
    BOOLCONST(bool),
    STRCONST(String),
    NULL,
    FOR,        // for keyword, used in list comprehension
    IN,         // in keyword, used in list comprehension
    IF,         // if keyword, used in list comprehension
    ADD,        // +
    MINUS,      // -
    MUL,        // *
    DIV,        // /
    MOD,        // %
    NOT,        // not
    AND,        // and
    OR,         // or
    EQ,         // ==
    NE,         // !=
    LT,         // <
    LE,         // <=
    GT,         // >
    GE,         // >=
    ID(String), // identifier
}

impl Token {
    pub fn to_str(&self) -> String {
        match self {
            Token::LBRAC => String::from("{"),
            Token::RBRAC => String::from("}"),
            Token::LSQBRAC => String::from("["),
            Token::RSQBRAC => String::from("]"),
            Token::LPAREN => String::from("("),
            Token::RPAREN => String::from(")"),
            Token::COLON => String::from(":"),
            Token::COMMA => String::from(","),
            Token::INTCONST(i) => i.to_string(),
            Token::BOOLCONST(b) => b.to_string(),
            Token::STRCONST(val) => String::from(val),
            Token::NULL => String::from("null"),
            Token::FOR => String::from("for"),
            Token::IN => String::from("in"),
            Token::IF => String::from("if"),
            Token::ADD => String::from("+"),
            Token::MINUS => String::from("-"),
            Token::MUL => String::from("*"),
            Token::DIV => String::from("/"),
            Token::MOD => String::from("%"),
            Token::NOT => String::from("not"),
            Token::AND => String::from("and"),
            Token::OR => String::from("or"),
            Token::EQ => String::from("=="),
            Token::NE => String::from("!="),
            Token::LT => String::from("<"),
            Token::LE => String::from("<="),
            Token::GT => String::from(">"),
            Token::GE => String::from(">="),
            Token::ID(name) => String::from(name),
        }
    }
}
