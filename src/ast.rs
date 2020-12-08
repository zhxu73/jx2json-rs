use std::collections::HashMap;
use std::fmt;

pub enum AstNode {
    INTVAL(i32),
    DOUBLEVAL(f64),
    STRVAL(String),
    BOOLVAL(bool),
    NULLVAL,
    // key value pairs
    OBJECT(HashMap<String, Box<AstNode>>),
    LIST(Vec<Box<AstNode>>),
    // Variable
    VAR(String),
    ADD {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    SUB {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    MUL {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    DIV {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    MOD {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    AND {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    OR {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    EQ {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    NE {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    GT {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    GE {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    LT {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    LE {
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    /// list comprehension
    COMPRE {
        expr: Box<AstNode>,
        var: String,
        iter_expr: Box<AstNode>,
    },
    FUNC {
        name: String,
        params: Vec<Box<AstNode>>,
    },
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AstNode::INTVAL(val) => write!(f, "{}", val),
            AstNode::DOUBLEVAL(val) => write!(f, "{}", val),
            AstNode::STRVAL(val) => write!(f, "{}", val),
            AstNode::BOOLVAL(val) => write!(f, "{}", val),
            AstNode::NULLVAL => write!(f, "null"),
            AstNode::OBJECT(keyval_list) => {
                if let Err(err) = write!(f, "{{ ") {
                    return Err(err);
                }
                for (key, value) in keyval_list {
                    if let Err(err) = write!(f, "\"{}\" : {}", key, value) {
                        return Err(err);
                    }
                }
                write!(f, " }}")
            }
            AstNode::LIST(list) => {
                if let Err(err) = write!(f, "[ ") {
                    return Err(err);
                }
                for node in list {
                    if let Err(err) = write!(f, "{}", node) {
                        return Err(err);
                    }
                }
                write!(f, " ]")
            }
            AstNode::VAR(name) => write!(f, "{}", name),
            AstNode::ADD { left, right }
            | AstNode::SUB { left, right }
            | AstNode::MUL { left, right }
            | AstNode::DIV { left, right }
            | AstNode::MOD { left, right }
            | AstNode::AND { left, right }
            | AstNode::OR { left, right }
            | AstNode::EQ { left, right }
            | AstNode::NE { left, right }
            | AstNode::GT { left, right }
            | AstNode::GE { left, right }
            | AstNode::LT { left, right }
            | AstNode::LE { left, right } => {
                write!(f, "{} {} {}", left, self.operator_str(), right)
            }
            AstNode::COMPRE {
                expr,
                var,
                iter_expr,
            } => write!(f, "[ {} for {} in {} ]", expr, var, iter_expr),
            AstNode::FUNC { name, params } => {
                if let Err(err) = write!(f, "{}(", name) {
                    return Err(err);
                }
                // all params except last one
                for param in params.iter().take(params.len() - 1) {
                    if let Err(err) = write!(f, "{}, ", param) {
                        return Err(err);
                    }
                }
                // last param
                match params.iter().last() {
                    Some(param) => {
                        if let Err(err) = write!(f, "{}", param) {
                            return Err(err);
                        }
                    }
                    None => (),
                };
                write!(f, ")")
            }
        }
    }
}

impl AstNode {
    fn operator_str(&self) -> &str {
        match &self {
            AstNode::ADD { left: _, right: _ } => "+",
            AstNode::SUB { left: _, right: _ } => "-",
            AstNode::MUL { left: _, right: _ } => "*",
            AstNode::DIV { left: _, right: _ } => "/",
            AstNode::MOD { left: _, right: _ } => "%",
            AstNode::AND { left: _, right: _ } => "&&",
            AstNode::OR { left: _, right: _ } => "||",
            AstNode::EQ { left: _, right: _ } => "==",
            AstNode::NE { left: _, right: _ } => "!=",
            AstNode::GT { left: _, right: _ } => ">",
            AstNode::GE { left: _, right: _ } => ">=",
            AstNode::LT { left: _, right: _ } => "<",
            AstNode::LE { left: _, right: _ } => "<=",
            _ => panic!(),
        }
    }

    pub fn is_int(&self) -> bool {
        match &self {
            AstNode::INTVAL(_) => true,
            _ => false,
        }
    }

    pub fn is_double(&self) -> bool {
        match &self {
            AstNode::DOUBLEVAL(_) => true,
            _ => false,
        }
    }

    pub fn is_str(&self) -> bool {
        match &self {
            AstNode::STRVAL(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match &self {
            AstNode::BOOLVAL(_) => true,
            _ => false,
        }
    }

    pub fn is_obj(&self) -> bool {
        match &self {
            AstNode::OBJECT(_) => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match &self {
            AstNode::LIST(_) => true,
            _ => false,
        }
    }

    pub fn is_var(&self) -> bool {
        match &self {
            AstNode::VAR(_) => true,
            _ => false,
        }
    }

    pub fn is_list_compre(&self) -> bool {
        match &self {
            AstNode::COMPRE {
                expr: _,
                var: _,
                iter_expr: _,
            } => true,
            _ => false,
        }
    }
}
