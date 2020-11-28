use super::jx_token::Token;
use std::fmt;
use std::fs;

pub struct ScannerError {
    reason: String,
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

pub fn scan_file(filename: &String) -> Result<Vec<Token>, ScannerError> {
    let contents = fs::read_to_string(filename).expect("Error when reading the file");
    scan_token(contents)
}

pub fn scan_token(raw: String) -> Result<Vec<Token>, ScannerError> {
    // reverse the input, so that the input is a stack/Vec whose top is the start of the input
    let mut input: String = raw.chars().rev().collect();
    let mut matching = String::from("");
    let mut tokens: Vec<Token> = vec![];

    if !move_forward(&mut input, &mut matching) {
        return Err(ScannerError {
            reason: format!("can not read characters from input {}", input),
        });
    }

    let mut back_track = false;
    loop {
        match token_match(&matching) {
            MatchResult::Matched(token) => {
                println!("token matched: {}", token.to_str());
                tokens.push(token);
                matching.clear();
            }
            MatchResult::More(opt) => {
                if back_track {
                    back_track = false; // only back track 1 char
                    match opt {
                        Some(token) => {
                            println!("token matched: {}", token.to_str());
                            tokens.push(token);
                            matching.clear();
                        }
                        None => {
                            return Err(ScannerError {
                                reason: format!("unable to match \"{}\"", matching),
                            })
                        }
                    }
                }
            }
            MatchResult::No => {
                if back_track {
                    // if already back tracking, then no match
                    return Err(ScannerError {
                        reason: format!("unable to match \"{}\"", matching),
                    });
                } else {
                    // start back tracking
                    if move_backward(&mut input, &mut matching) {
                        return Err(ScannerError {
                            reason: format!("unable to match \"{}\"", matching),
                        });
                    }
                    back_track = true;
                    continue;
                }
            }
        };
        if !move_forward(&mut input, &mut matching) {
            break;
        }
    }
    Ok(tokens)
}

// move forward 1 char
fn move_forward(src: &mut String, matching: &mut String) -> bool {
    let mut c = match src.pop() {
        Some(c) => c,
        None => return false,
    };

    match matching.chars().nth(0) {
        Some(first) => {
            if first == '\'' || first == '"' {
                matching.push(c);
                return true;
            }
        }
        None => (),
    };

    // skip whitespaces char if not in String Const
    while c.is_whitespace() {
        c = match src.pop() {
            Some(c) => c,
            None => return false,
        };
    }
    matching.push(c);
    true
}

// move backward 1 char
fn move_backward(src: &mut String, matching: &mut String) -> bool {
    let c = match matching.pop() {
        Some(c) => c,
        None => return false,
    };
    src.push(c);
    true
}

enum MatchResult {
    // matched, no need to read further
    Matched(Token),
    // need to read further
    More(Option<Token>),
    // not matched, no need to read further
    No,
}

fn token_match(input: &str) -> MatchResult {
    let c = match input.chars().nth(0) {
        Some(c) => c,
        None => return MatchResult::No,
    };
    println!("t_m(), input {}", input);
    match c {
        '{' => match_1_char_token(input, Token::LBRAC),
        '}' => match_1_char_token(input, Token::RBRAC),
        '[' => match_1_char_token(input, Token::LSQBRAC),
        ']' => match_1_char_token(input, Token::RSQBRAC),
        '(' => match_1_char_token(input, Token::LPAREN),
        ')' => match_1_char_token(input, Token::RPAREN),
        ':' => match_1_char_token(input, Token::COLON),
        ',' => match_1_char_token(input, Token::COMMA),
        '+' => match_1_char_token(input, Token::ADD),
        '-' => match_negative_or_minus(input),
        '*' => match_1_char_token(input, Token::MUL),
        '/' => match_1_char_token(input, Token::DIV),
        '%' => match_1_char_token(input, Token::MOD),
        '=' => {
            if input.eq("==") {
                MatchResult::Matched(Token::EQ)
            } else {
                MatchResult::More(None)
            }
        }
        '!' => {
            if input.eq("!=") {
                MatchResult::Matched(Token::NE)
            } else {
                MatchResult::More(None)
            }
        }
        '<' => {
            if input.eq("<=") {
                MatchResult::Matched(Token::LE)
            } else if input.len() == 1 {
                MatchResult::More(Some(Token::LT))
            } else {
                MatchResult::No
            }
        }
        '>' => {
            if input.starts_with(">=") {
                MatchResult::Matched(Token::GE)
            } else if input.len() == 1 {
                MatchResult::More(Some(Token::GT))
            } else {
                MatchResult::No
            }
        }
        '0'..='9' => match_numeric(input),
        // ID
        'a'..='z' => match_alphabetic(input),
        'A'..='Z' => match_id(input),

        // String literal
        '\'' => match_strconst(input, '\''),
        '"' => match_strconst(input, '"'),
        _ => MatchResult::No,
    }
}

fn match_numeric(input: &str) -> MatchResult {
    match input.parse::<i32>().ok() {
        Some(val) => return MatchResult::More(Some(Token::INTCONST(val))),
        None => (),
    };
    // not Matched ending with dot, e.g. "123."
    match input.chars().last() {
        Some(c) => {
            if c == '.' {
                match &input[..input.len() - 1].parse::<i32>().ok() {
                    Some(_) => return MatchResult::More(None),
                    None => return MatchResult::No,
                };
            }
        }
        None => (),
    };
    match input.parse::<f64>().ok() {
        Some(val) => return MatchResult::More(Some(Token::DOUBLECONST(val))),
        None => (),
    }

    MatchResult::No
}

// assume input[0] == '-'
fn match_negative_or_minus(input: &str) -> MatchResult {
    if input.len() == 1 {
        return MatchResult::Matched(Token::MINUS);
    }
    let opt = match match_numeric(&input[1..]) {
        MatchResult::More(opt) => opt,
        _ => return MatchResult::No,
    };
    let token = match opt {
        Some(token) => token,
        None => return MatchResult::No,
    };
    match token {
        Token::INTCONST(val) => MatchResult::More(Some(Token::INTCONST(-val))),
        Token::DOUBLECONST(val) => MatchResult::More(Some(Token::DOUBLECONST(-val))),
        _ => MatchResult::No,
    }
}

// match token that only consist of 1 character
fn match_1_char_token(input: &str, token: Token) -> MatchResult {
    if input.len() > 1 {
        return MatchResult::No;
    }
    MatchResult::Matched(token)
}

// match anything start with a alphabet
fn match_alphabetic(input: &str) -> MatchResult {
    let c = match input.chars().nth(0) {
        Some(c) => c,
        None => return MatchResult::No,
    };
    match c {
        'f' => {
            if input == "false" {
                return MatchResult::More(Some(Token::BOOLCONST(false)));
            } else if input == "for" {
                return MatchResult::More(Some(Token::FOR));
            }
        }
        't' => {
            if input == "true" {
                return MatchResult::More(Some(Token::BOOLCONST(true)));
            }
        }
        'n' => {
            if input == "null" {
                return MatchResult::More(Some(Token::NULL));
            }
        }
        'i' => {
            if input == "in" {
                return MatchResult::More(Some(Token::IN));
            }
        }
        _ => (),
    };
    match_id(input)
}

// assume the 1st char is a letter
fn match_id(input: &str) -> MatchResult {
    for c in input.chars() {
        if c.is_whitespace() {
            return MatchResult::No;
        }
        match c {
            'a'..='z' => continue,
            'A'..='Z' => continue,
            '0'..='9' => continue,
            '_' => continue,
            _ => return MatchResult::No,
        }
    }
    MatchResult::More(Some(Token::ID(String::from(input))))
}

fn match_strconst(input: &str, quote: char) -> MatchResult {
    if input.ends_with(quote) && input.len() > 1 {
        let str_const = &input[1..input.len() - 1];
        MatchResult::Matched(Token::STRCONST(String::from(str_const)))
    } else {
        MatchResult::More(None)
    }
}

#[cfg(test)]
mod tests {
    use super::scan_token;
    use super::token_match;
    use super::MatchResult;
    use super::Token;

    #[test]
    fn match_intconst1() {
        let input = String::from("9");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                _ => panic!("Should match MORE(token)"),
            },
            _ => panic!("Should match MORE"),
        };
        match token {
            Token::INTCONST(val) => assert_eq!(val, 9),
            _ => panic!("Should match INTCONST"),
        }
    }

    #[test]
    fn match_intconst2() {
        let input = String::from("12345abc");
        match token_match(&input) {
            MatchResult::No => return,
            _ => panic!("Should not match"),
        };
    }

    #[test]
    fn match_intconst3() {
        let input = String::from("143241");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                _ => panic!("Should match MORE(token)"),
            },
            _ => panic!("Should match MORE"),
        };
        match token {
            Token::INTCONST(val) => assert_eq!(val, 143241),
            _ => panic!("Should match INTCONST"),
        }
    }

    #[test]
    fn match_negative_intconst() {
        let input = String::from("-534");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                _ => panic!("Should match MORE(token)"),
            },
            _ => panic!("Should match MORE"),
        };
        match token {
            Token::INTCONST(val) => assert_eq!(val, -534),
            _ => panic!("Should match INTCONST"),
        }
    }

    #[test]
    fn match_strconst_single_quotes() {
        let input = String::from("'foo bar'");
        let token = match token_match(&input) {
            MatchResult::Matched(token) => token,
            _ => panic!("Should Matched"),
        };
        match token {
            Token::STRCONST(val) => assert_eq!(val, "foo bar"),
            _ => panic!("Should match STRCONST"),
        }
    }

    #[test]
    fn match_strconst_double_quotes() {
        let input = String::from("\"foo bar\"");
        let token = match token_match(&input) {
            MatchResult::Matched(token) => token,
            _ => panic!("Should Matched"),
        };
        match token {
            Token::STRCONST(val) => assert_eq!(val, "foo bar"),
            _ => panic!("Should match STRCONST"),
        }
    }

    #[test]
    fn match_strconst_mixed_quotes1() {
        let input = String::from("'foo_bar\"");
        match token_match(&input) {
            MatchResult::More(opt) => match opt {
                None => return,
                _ => panic!("Should matched MORE(None)"),
            },
            _ => panic!("Should matched More"),
        };
    }

    #[test]
    fn match_strconst_mixed_quotes2() {
        let input = String::from("\"foo_bar'");
        match token_match(&input) {
            MatchResult::More(opt) => match opt {
                None => return,
                _ => panic!("Should matched MORE(None)"),
            },
            _ => panic!("Should matched More"),
        };
    }

    #[test]
    fn match_id1() {
        let input = String::from("a");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                _ => panic!("Should match MORE(token)"),
            },
            _ => panic!("Should match MORE"),
        };
        match token {
            Token::ID(val) => assert_eq!(val, "a"),
            _ => panic!("Should match ID"),
        }
    }

    #[test]
    fn match_id2() {
        let input = String::from("foo_bar");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                _ => panic!("Should match MORE(token)"),
            },
            _ => panic!("Should match MORE"),
        };
        match token {
            Token::ID(val) => assert_eq!(val, "foo_bar"),
            _ => panic!("Should match ID"),
        }
    }

    #[test]
    fn match_id3() {
        let input = String::from("FOO_BAR");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                _ => panic!("Should match MORE(token)"),
            },
            _ => panic!("Should match MORE"),
        };
        match token {
            Token::ID(val) => assert_eq!(val, "FOO_BAR"),
            _ => panic!("Should match ID"),
        }
    }

    #[test]
    fn match_id_whitespace() {
        let input = String::from("foo bar");
        match token_match(&input) {
            MatchResult::No => return,
            _ => panic!("Should Not matched"),
        };
    }

    #[test]
    fn match_bool_true() {
        let input = String::from("true");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                None => panic!("Should matched More(token)"),
            },
            _ => panic!("Should matched More"),
        };
        match token {
            Token::BOOLCONST(true) => return,
            Token::BOOLCONST(false) => panic!("Should match true"),
            _ => panic!("Should match BOOLCONST"),
        }
    }

    #[test]
    fn match_bool_false() {
        let input = String::from("false");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                None => panic!("Should matched More(token)"),
            },
            _ => panic!("Should matched More"),
        };
        match token {
            Token::BOOLCONST(true) => panic!("should match false"),
            Token::BOOLCONST(false) => return,
            _ => panic!("Should match BOOLCONST"),
        }
    }

    #[test]
    fn match_double_zero() {
        let input = String::from("0.0");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                None => panic!("Should matched More(token)"),
            },
            _ => panic!("Should matched More"),
        };
        match token {
            Token::DOUBLECONST(f) => assert_eq!(0.0, f),
            _ => panic!("Should match DOUBLECONST"),
        }
    }

    #[test]
    fn match_double1() {
        let input = String::from("1.0");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                None => panic!("Should matched More(token)"),
            },
            _ => panic!("Should matched More"),
        };
        match token {
            Token::DOUBLECONST(f) => assert_eq!(1.0, f),
            _ => panic!("Should match DOUBLECONST"),
        }
    }

    #[test]
    fn match_double2() {
        let input = String::from("2.14523124695");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                None => panic!("Should matched More(token)"),
            },
            _ => panic!("Should matched More"),
        };
        match token {
            Token::DOUBLECONST(f) => assert_eq!(2.14523124695, f),
            _ => panic!("Should match DOUBLECONST"),
        }
    }

    #[test]
    fn match_double_end_with_dot() {
        let input = String::from("2.");
        match token_match(&input) {
            MatchResult::More(opt) => match opt {
                None => return,
                Some(_) => panic!("Should matched More(None)"),
            },
            _ => panic!("Should matched More"),
        };
    }

    #[test]
    fn match_null() {
        let input = String::from("null");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                _ => panic!("Should matched More(token)"),
            },
            _ => panic!("Should matched More"),
        };
        match token {
            Token::NULL => return,
            _ => panic!("Should matched NULL"),
        }
    }

    #[test]
    fn match_null_cap() {
        let input = String::from("NULL");
        let token = match token_match(&input) {
            MatchResult::More(opt) => match opt {
                Some(token) => token,
                _ => panic!("Should matched More(token)"),
            },
            _ => panic!("Should matched More"),
        };
        match token {
            Token::ID(name) => assert_eq!("NULL", name),
            _ => panic!("Should matched ID"),
        }
    }

    #[test]
    fn scan_tokens1() {
        let input = String::from("{\"rules\": []}");
        let result = scan_token(input);
        let expected = vec![
            Token::LBRAC,
            Token::STRCONST(String::from("rules")),
            Token::COLON,
            Token::LSQBRAC,
            Token::RSQBRAC,
            Token::RBRAC,
        ];

        match result {
            Ok(tokens) => {
                let matching = tokens
                    .iter()
                    .zip(&expected)
                    .filter(|&(token, expect)| token == expect)
                    .count();
                assert_eq!(tokens.len(), matching);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                panic!("Should not have error")
            }
        }
    }
}
