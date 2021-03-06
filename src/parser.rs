use super::ast::AstNode;
use super::jx_token::Token;
use super::symbol_tab::SymbolTable;
use std::{collections::HashMap, fmt};
use std::{error::Error, vec};

type Ast = Box<AstNode>;

pub struct ParserResult {
    pub root: Ast,
    pub tab: SymbolTable,
}

#[derive(Debug)]
pub struct ParserErr {}

impl Error for ParserErr {}

impl fmt::Display for ParserErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parser error")
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<ParserResult, ParserErr> {
    let mut src = TokenSrc::from(tokens);
    match parse_workflow(&mut src) {
        Some(node) => {
            return Ok(ParserResult {
                root: node,
                tab: SymbolTable::new(),
            })
        }
        None => {
            return Err(ParserErr {});
        }
    }
}

struct TokenSrc {
    tokens: Vec<Token>,
    curr_index: usize,
}
impl TokenSrc {
    fn from(tokens: Vec<Token>) -> TokenSrc {
        TokenSrc {
            tokens,
            curr_index: 0,
        }
    }

    fn consume(&mut self) {
        if self.curr_index + 1 > self.tokens.len() {
            panic!("consume out of bound")
        }
        println!("consume {}", self.tokens[self.curr_index]);
        self.curr_index += 1;
    }

    fn backtrack_one(&mut self) {
        if self.curr_index < 1 {
            panic!("backtrack_one out of bound")
        }
        self.curr_index -= 1;
    }

    fn backtrack_to(&mut self, index: usize) {
        if self.curr_index < index {
            panic!("backtrack_to cannot be used to advance")
        }
        self.curr_index = index;
    }

    fn curr(&self) -> &Token {
        self.tokens.get(self.curr_index).unwrap()
    }

    fn curr_token(&self) -> Token {
        self.tokens.get(self.curr_index).unwrap().clone()
    }

    fn next_token(&self) -> Token {
        self.tokens.get(self.curr_index + 1).unwrap().clone()
    }

    fn curr_index(&self) -> usize {
        self.curr_index
    }
}

fn parse_workflow(src: &mut TokenSrc) -> Option<Ast> {
    match_object(src)
}

fn match_key_val_list(
    src: &mut TokenSrc,
    mut keyval_pairs: HashMap<String, Ast>,
) -> Option<HashMap<String, Ast>> {
    match match_key_val(src) {
        Some((k, v)) => {
            keyval_pairs.insert(k, v);
        }
        None => return None,
    }

    match src.curr() {
        // more key_val
        Token::COMMA => (),
        _ => return Some(keyval_pairs),
    }
    if !match_terminal(src, Token::COMMA) {
        // key_val
        return None;
    }

    match match_key_val_list(src, keyval_pairs) {
        Some(pairs) => return Some(pairs),
        None => return None,
    }
}

fn match_key_val(src: &mut TokenSrc) -> Option<(String, Ast)> {
    let key = match match_STRVAL(src) {
        Some(key) => key,
        None => return None,
    };
    if !match_terminal(src, Token::COLON) {
        return None;
    }
    let value = match match_expr(src) {
        Some(val) => val,
        None => return None,
    };
    Some((key, value))
}

fn match_expr(src: &mut TokenSrc) -> Option<Ast> {
    match src.curr() {
        // both jx_expr and value
        Token::STRCONST(_)
        | Token::INTCONST(_)
        | Token::DOUBLECONST(_)
        | Token::BOOLCONST(_)
        | Token::LBRAC
        | Token::LSQBRAC => (),
        // jx_expr only
        Token::ID(_) => return match_jx_expr(src),
        _ => return None,
    }

    let index = src.curr_index();
    match match_jx_expr(src) {
        Some(expr) => return Some(expr),
        None => src.backtrack_to(index),
    }
    println!("backtracked jx_expr");
    match_value(src)
}

fn match_value(src: &mut TokenSrc) -> Option<Ast> {
    match src.curr() {
        Token::STRCONST(val) => {
            let node = Box::new(AstNode::STRVAL(val.clone()));
            src.consume();
            Some(node)
        }
        Token::INTCONST(val) => {
            let node = Box::new(AstNode::INTVAL(val.clone()));
            src.consume();
            Some(node)
        }
        Token::DOUBLECONST(val) => {
            let node = Box::new(AstNode::DOUBLEVAL(val.clone()));
            src.consume();
            Some(node)
        }
        Token::BOOLCONST(val) => {
            let node = Box::new(AstNode::BOOLVAL(val.clone()));
            src.consume();
            Some(node)
        }
        Token::LBRAC => return match_object(src),
        Token::LSQBRAC => return match_list(src),
        _ => return None,
    }
}

fn match_object(src: &mut TokenSrc) -> Option<Ast> {
    if !match_terminal(src, Token::LBRAC) {
        return None;
    }

    let keyval_pairs: HashMap<String, Box<AstNode>> = HashMap::new();
    match src.curr() {
        // empty object
        Token::RBRAC => return Some(Box::new(AstNode::OBJECT(keyval_pairs))),
        _ => (),
    };

    let keyval_pairs = match match_key_val_list(src, keyval_pairs) {
        Some(keyval_list) => keyval_list,
        None => return None,
    };

    let obj = Box::new(AstNode::OBJECT(keyval_pairs));
    if !match_terminal(src, Token::RBRAC) {
        return None;
    }
    Some(obj)
}

fn match_list(src: &mut TokenSrc) -> Option<Ast> {
    let list;

    if !match_terminal(src, Token::LSQBRAC) {
        return None;
    }
    match src.curr() {
        // empty list
        Token::RSQBRAC => list = Box::new(AstNode::LIST(vec![])),
        // non-empty list
        _ => {
            let expr_list = vec![];
            let expr_list = match match_expr_list(src, expr_list) {
                Some(expr_list) => expr_list,
                None => return None,
            };
            list = Box::new(AstNode::LIST(expr_list));
        }
    };

    if !match_terminal(src, Token::RSQBRAC) {
        return None;
    }
    Some(list)
}

fn match_expr_list(src: &mut TokenSrc, mut list: Vec<Ast>) -> Option<Vec<Ast>> {
    let expr = match match_expr(src) {
        Some(expr) => expr,
        None => return None,
    };
    list.push(expr);

    match src.curr() {
        // more expr
        Token::COMMA => (),
        // no more expr, return the matched one
        _ => return Some(list),
    };

    if !match_terminal(src, Token::COMMA) {
        return None;
    }

    let list = match match_expr_list(src, list) {
        Some(list) => list,
        None => return None,
    };
    Some(list)
}

fn match_jx_expr(src: &mut TokenSrc) -> Option<Ast> {
    println!("\t\t{}", src.next_token());
    println!("jx_expr");
    match src.curr() {
        // both jx_arith_expr | list_compre_expr
        Token::STRCONST(_) | Token::INTCONST(_) | Token::DOUBLECONST(_) => {
            let index = src.curr_index();
            match match_jx_arith_expr(src) {
                Some(ast) => return Some(ast),
                None => src.backtrack_to(index),
            }
            println!("backtrack jx_arith_expr");
            match_list_compre_expr(src)
        }
        // list_compre_expr only
        Token::BOOLCONST(_) | Token::LBRAC | Token::LSQBRAC => match_list_compre_expr(src),
        _ => None,
    }
}

fn match_jx_arith_expr(src: &mut TokenSrc) -> Option<Ast> {
    match src.curr() {
        Token::STRCONST(_) => {
            let s1 = match match_STRVAL(src) {
                Some(val) => val,
                None => return None,
            };
            if !match_terminal(src, Token::ADD) {
                return None;
            }
            let s2 = match match_STRVAL(src) {
                Some(val) => val,
                None => return None,
            };
            Some(Box::new(AstNode::ADD {
                left: Box::new(AstNode::STRVAL(s1)),
                right: Box::new(AstNode::STRVAL(s2)),
            }))
        }
        Token::INTCONST(_) => {
            let i1 = match match_int(src) {
                Some(val) => val,
                None => return None,
            };
            if !match_terminal(src, Token::ADD) {
                return None;
            }
            let i2 = match match_int(src) {
                Some(val) => val,
                None => return None,
            };
            Some(Box::new(AstNode::ADD {
                left: Box::new(AstNode::INTVAL(i1)),
                right: Box::new(AstNode::INTVAL(i2)),
            }))
        }
        Token::DOUBLECONST(_) => {
            let d1 = match match_double(src) {
                Some(val) => val,
                None => return None,
            };
            if !match_terminal(src, Token::ADD) {
                return None;
            }
            let d2 = match match_double(src) {
                Some(val) => val,
                None => return None,
            };
            return Some(Box::new(AstNode::ADD {
                left: Box::new(AstNode::DOUBLEVAL(d1)),
                right: Box::new(AstNode::DOUBLEVAL(d2)),
            }));
        }
        _ => return None,
    }
}

fn match_list_compre_expr(src: &mut TokenSrc) -> Option<Ast> {
    println!("list_compre_expr");
    let expr = match match_value(src) {
        Some(val) => val,
        None => return None,
    };

    if !match_terminal(src, Token::FOR) {
        return None;
    }
    let variable = match match_id(src) {
        Some(val) => val,
        None => return None,
    };
    if !match_terminal(src, Token::IN) {
        return None;
    }
    // iterable_expr
    let list = match match_list(src) {
        Some(val) => val,
        None => return None,
    };
    // opt_list_compre_expr
    // FIXME
    //match match_opt_list_compre_expr(src) {};

    Some(Box::new(AstNode::COMPRE {
        expr: expr,
        var: "".to_string(), // FIXME use variable
        iter_expr: list,
    }))
}

fn match_opt_list_compre_expr(src: &mut TokenSrc) -> Option<Ast> {
    if !match_terminal(src, Token::FOR) {
        return None;
    }
    let variable = match match_id(src) {
        Some(val) => val,
        None => return None,
    };
    if !match_terminal(src, Token::IN) {
        return None;
    }
    // iterable_expr
    let list = match match_list(src) {
        Some(val) => val,
        None => return None,
    };
    // opt_list_compre_expr
    // FIXME
    //match match_opt_list_compre_expr(src) {};

    // FIXME
    None
}

fn parse_jx_expr(src: &TokenSrc) {
    let token = src.curr();
    match token {
        Token::STRCONST(val) => (),
        Token::INTCONST(val) => (),
        _ => panic!(),
    };
}

/// match terminal/token, consume the token if matched
fn match_terminal(src: &mut TokenSrc, terminal: Token) -> bool {
    if src.curr() == &terminal {
        src.consume();
        return true;
    }
    false
}

fn match_STRCONST(src: &mut TokenSrc) -> bool {
    match src.curr() {
        Token::STRCONST(val) => {
            src.consume();
            return true;
        }
        _ => return false,
    };
}

fn match_STRVAL(src: &mut TokenSrc) -> Option<String> {
    let val = match src.curr() {
        Token::STRCONST(val) => {
            let new_val = val.clone();
            src.consume();
            return Some(new_val);
        }
        _ => return None,
    };
}

fn match_int(src: &mut TokenSrc) -> Option<i32> {
    let val = match src.curr() {
        Token::INTCONST(val) => {
            let new_val = val.clone();
            src.consume();
            return Some(new_val);
        }
        _ => return None,
    };
}

fn match_double(src: &mut TokenSrc) -> Option<f64> {
    let val = match src.curr() {
        Token::DOUBLECONST(val) => {
            let new_val = val.clone();
            src.consume();
            return Some(new_val);
        }
        _ => return None,
    };
}

fn match_id(src: &mut TokenSrc) -> Option<Ast> {
    match src.curr() {
        Token::ID(name) => {
            let id = name.clone();
            src.consume();
            Some(Box::new(AstNode::VAR(id)))
        }
        _ => None,
    }
}
