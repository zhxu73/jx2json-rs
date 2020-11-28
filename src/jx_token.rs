use std::fmt;

#[derive(PartialEq, Clone)]
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
    DOUBLECONST(f64),
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
            Token::DOUBLECONST(f) => f.to_string(),
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LBRAC => write!(f, "LBRAC"),
            Token::RBRAC => write!(f, "RBRAC"),
            Token::LSQBRAC => write!(f, "LSQBRAC"),
            Token::RSQBRAC => write!(f, "RSQBRAC"),
            Token::LPAREN => write!(f, "LPAREN"),
            Token::RPAREN => write!(f, "RPAREN"),
            Token::COLON => write!(f, "COLON"),
            Token::COMMA => write!(f, "COMMA"),
            Token::INTCONST(val) => write!(f, "INTCONST({})", val),
            Token::DOUBLECONST(val) => write!(f, "DOUBLECONST({})", val),
            Token::BOOLCONST(val) => write!(f, "  BOOLCONST({})", val),
            Token::STRCONST(val) => write!(f, "STRCONST({})", val),
            Token::NULL => write!(f, "NULL"),
            Token::FOR => write!(f, "FOR"),
            Token::IN => write!(f, "IN"),
            Token::IF => write!(f, "IF"),
            Token::ADD => write!(f, "ADD"),
            Token::MINUS => write!(f, "MINUS"),
            Token::MUL => write!(f, "MUL"),
            Token::DIV => write!(f, "DIV"),
            Token::MOD => write!(f, "MOD"),
            Token::NOT => write!(f, "NOT"),
            Token::AND => write!(f, "AND"),
            Token::OR => write!(f, "OR"),
            Token::EQ => write!(f, "EQ"),
            Token::NE => write!(f, "NE"),
            Token::LT => write!(f, "LT"),
            Token::LE => write!(f, "LE"),
            Token::GT => write!(f, "GT"),
            Token::GE => write!(f, "GE"),
            Token::ID(name) => write!(f, "ID({})", name),
        }
    }
}
