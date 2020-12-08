extern crate jx2json;
use jx2json::parser;
use jx2json::{ast::AstNode, jx_token::Token};

#[test]
fn parse_empty_workflow1() {
    let input = vec![Token::LBRAC, Token::RBRAC];
    let result = match parser::parse_tokens(input) {
        Ok(result) => result,
        Err(err) => panic!("{}", err),
    };
    if let AstNode::OBJECT(keyval_list) = result.root.as_ref() {
        assert!(keyval_list.is_empty());
    } else {
        panic!("{} should be OBJECT", result.root);
    }
    assert!(result.tab.count() == 0);
}

#[test]
fn parse_empty_workflow2() {
    let input = vec![
        Token::LBRAC,
        Token::STRCONST(String::from("rules")),
        Token::COLON,
        Token::LSQBRAC,
        Token::RSQBRAC,
        Token::RBRAC,
    ];
    let result = match parser::parse_tokens(input) {
        Ok(result) => result,
        Err(err) => panic!("{}", err),
    };
    if let AstNode::OBJECT(keyval_pairs) = result.root.as_ref() {
        assert_eq!(1, keyval_pairs.len());
        for (key, val) in keyval_pairs {
            assert_eq!("rules", key);
            match val.as_ref() {
                // empty list
                AstNode::LIST(list) => assert!(list.is_empty()),
                _ => panic!("{} should be LIST", val),
            };
        }
    } else {
        panic!("{} should be OBJECT", result.root);
    }
    assert_eq!(0, result.tab.count());
}

#[test]
fn parse_workflow1() {
    let input = vec![
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
    let result = match parser::parse_tokens(input) {
        Ok(result) => result,
        Err(err) => panic!("{}", err),
    };
    let rule = check_if_1_rule_wf(&result.root);

    // rule has 3 keys (command, outputs, inputs)
    let keyval_pairs = match rule {
        AstNode::OBJECT(keyval_pairs) => {
            assert_eq!(3, keyval_pairs.len());
            keyval_pairs
        }
        _ => panic!("{} should be OBJECT"),
    };
    for (key, val) in keyval_pairs {
        match key.as_str() {
            // command should be a string
            "command" => match val.as_ref() {
                AstNode::STRVAL(cmd) => assert_eq!("/bin/echo 'foo bar' > out.txt", cmd),
                _ => panic!("{} should be STRVAL", val),
            },
            // outputs should be list contains 1 string
            "outputs" => match val.as_ref() {
                AstNode::LIST(outputs) => {
                    assert_eq!(1, outputs.len());
                    let elem = outputs.first().unwrap();
                    match elem.as_ref() {
                        AstNode::STRVAL(val) => assert_eq!("out.txt", val),
                        _ => panic!("{} should be STRVAL", elem),
                    }
                }
                _ => panic!("{} should be STRVAL", val),
            },
            // inputs should be empty list
            "inputs" => match val.as_ref() {
                AstNode::LIST(inputs) => assert!(inputs.is_empty()),
                _ => panic!("{} should be STRVAL", val),
            },
            _ => (),
        };
    }

    assert!(result.tab.count() == 0);
}

/*
#[test]
fn parse_workflow2() {
    let input = vec![
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
    let result = match parser::parse_tokens(input) {
        Ok(result) => result,
        Err(err) => panic!("{}", err),
    };
    let rule = check_if_1_rule_wf(&result.root);

    // rule has 3 keys (command, outputs, inputs)
    let keyval_pairs = match rule {
        AstNode::OBJECT(keyval_pairs) => {
            assert_eq!(3, keyval_pairs.len());
            keyval_pairs
        }
        _ => panic!("{} should be OBJECT"),
    };
    for (key, val) in keyval_pairs {
        match key.as_str() {
            // command should be a string
            "command" => match val.as_ref() {
                AstNode::STRVAL(cmd) => assert_eq!("/bin/echo 'foo bar' > out.txt", cmd),
                _ => panic!("{} should be STRVAL", val),
            },
            // outputs should be list contains 1 element
            "outputs" => {
                assert!(val.is_list());
                if let AstNode::LIST(outputs) = val.as_ref() {
                    assert_eq!(1, outputs.len());
                    let elem = outputs.first().unwrap();
                    match elem.as_ref() {
                        AstNode::ADD { left, right } => {
                            assert!(left.as_ref().is_str());
                            if let AstNode::STRVAL(val) = left.as_ref() {
                                assert_eq!("out", val);
                            }
                            assert!(right.as_ref().is_str());
                            if let AstNode::STRVAL(val) = right.as_ref() {
                                assert_eq!(".txt", val);
                            }
                        }
                        _ => panic!("{} should be ADD", elem),
                    }
                }
            }
            // inputs should be list contains 1 element
            "inputs" => match val.as_ref() {
                AstNode::LIST(inputs) => {
                    assert_eq!(1, inputs.len());
                    let elem = inputs.first().unwrap();
                    assert!(elem.is_list_compre());
                    if let AstNode::COMPRE {
                        expr,
                        var,
                        iter_expr,
                    } = elem.as_ref()
                    {
                        match expr.as_ref() {
                            AstNode::ADD { left, right } => (/* FIXME */),
                            _ => panic!("{} should be ADD", expr),
                        };
                        assert_eq!("x1", var);
                        assert!(iter_expr.as_ref().is_list());
                    }
                }
                _ => panic!("{} should be LIST", val),
            },
            _ => (),
        };
    }
    assert!(result.tab.count() == 1);
}
*/

/// assert the wf only has "rules" key, and 1 rules
/// return the rule AstNode
fn check_if_1_rule_wf(root: &AstNode) -> &AstNode {
    // root should be object
    let keyval_pairs = match &root {
        AstNode::OBJECT(keyval_list) => keyval_list,
        _ => panic!("{} should be OBJECT", root),
    };
    // root should contains 1 key ("rules"), the value corrsponds to "rules" is a list with 1 elem
    assert_eq!(1, keyval_pairs.len());
    assert_eq!("rules", keyval_pairs.keys().next().unwrap());
    let rules = keyval_pairs.values().next().unwrap();
    let rule = match rules.as_ref() {
        // empty list
        AstNode::LIST(list) => {
            assert_eq!(1, list.len());
            list.first().unwrap()
        }
        _ => panic!("{} should be LIST", rules),
    };
    rule
}
