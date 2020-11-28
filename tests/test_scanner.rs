extern crate jx2json;

use jx2json::jx_token::Token;
use jx2json::scanner;

#[test]
fn scan_tokens1() {
    let input = String::from("{\"rules\": []}");
    let result = scanner::scan_token(input);
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
            assert_eq!(true, compare_tokens(&tokens, &expected));
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            panic!("Should not have error")
        }
    }
}

#[test]
fn scan_tokens2() {
    let raw = "{
    \"rules\": [
                {
                    \"command\" : \"/bin/echo 'foo bar' > out.txt\",
                    \"outputs\" : [ \"out.txt\" ],
                    \"inputs\"  : [ ]
                }
            ]
}";

    let input = String::from(raw);
    let result = scanner::scan_token(input);
    let expected = vec![
        Token::LBRAC,
        Token::STRCONST(String::from("rules")),
        Token::COLON,
        Token::LSQBRAC,
        Token::LBRAC,
        Token::STRCONST(String::from("command")),
        Token::COLON,
        Token::STRCONST(String::from("/bin/echo 'foo bar' > out.txt")),
        Token::COMMA,
        Token::STRCONST(String::from("outputs")),
        Token::COLON,
        Token::LSQBRAC,
        Token::STRCONST(String::from("out.txt")),
        Token::RSQBRAC,
        Token::COMMA,
        Token::STRCONST(String::from("inputs")),
        Token::COLON,
        Token::LSQBRAC,
        Token::RSQBRAC,
        Token::RBRAC,
        Token::RSQBRAC,
        Token::RBRAC,
    ];

    match result {
        Ok(tokens) => {
            assert_eq!(true, compare_tokens(&tokens, &expected));
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            panic!("Should not have error")
        }
    }
}

#[test]
fn scan_tokens3() {
    let raw = "{
    \"rules\": [
                {
                    \"command\" : \"/bin/echo 'foo bar' > out.txt\",
                    \"outputs\" : [ \"out\" + \".txt\" ],
                    \"inputs\"  : [ \"in\" + x1 + \".txt\" for x1 in [\"a\", \"b\", \"c\"] ]
                }
            ]
}";
    let input = String::from(raw);
    let result = scanner::scan_token(input);
    let expected = vec![
        Token::LBRAC,
        Token::STRCONST(String::from("rules")),
        Token::COLON,
        Token::LSQBRAC,
        Token::LBRAC,
        Token::STRCONST(String::from("command")),
        Token::COLON,
        Token::STRCONST(String::from("/bin/echo 'foo bar' > out.txt")),
        Token::COMMA,
        Token::STRCONST(String::from("outputs")),
        Token::COLON,
        Token::LSQBRAC,
        Token::STRCONST(String::from("out")),
        Token::ADD,
        Token::STRCONST(String::from(".txt")),
        Token::RSQBRAC,
        Token::COMMA,
        Token::STRCONST(String::from("inputs")),
        Token::COLON,
        Token::LSQBRAC,
        Token::STRCONST(String::from("in")),
        Token::ADD,
        Token::ID(String::from("x1")),
        Token::ADD,
        Token::STRCONST(String::from(".txt")),
        Token::FOR,
        Token::ID(String::from("x1")),
        Token::IN,
        Token::LSQBRAC,
        Token::STRCONST(String::from("a")),
        Token::COMMA,
        Token::STRCONST(String::from("b")),
        Token::COMMA,
        Token::STRCONST(String::from("c")),
        Token::RSQBRAC,
        Token::RSQBRAC,
        Token::RBRAC,
        Token::RSQBRAC,
        Token::RBRAC,
    ];

    match result {
        Ok(tokens) => {
            assert_eq!(true, compare_tokens(&tokens, &expected));
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            panic!("Should not have error")
        }
    }
}

fn compare_tokens(tokens: &Vec<Token>, expected: &Vec<Token>) -> bool {
    let matching = tokens
        .iter()
        .zip(expected)
        .filter(|&(token, expect)| {
            if token != expect {
                eprintln!("token mismatched, got {}, expect {}", token, expect);
            };
            token == expect
        })
        .count();
    tokens.len() == matching
}
