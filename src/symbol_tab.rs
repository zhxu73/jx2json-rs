use std::collections::HashMap;
use std::fmt;

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, symbol: Symbol) {
        self.symbols.insert(name, symbol);
    }

    pub fn find(&self, name: &String) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn count(&self) -> usize {
        self.symbols.len()
    }
}

pub enum Symbol {
    LIST(Vec<Symbol>),
    OBJECT(HashMap<String, Symbol>),
    INT(i32),
    DOUBLE(f64),
    BOOL(bool),
    FUNC { return_type: ValueType },
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Symbol::LIST(list) => {
                if let Err(err) = write!(f, "[ ") {
                    return Err(err);
                }
                for symbol in list {
                    if let Err(err) = write!(f, "{}", symbol) {
                        return Err(err);
                    }
                }
                write!(f, " ]")
            }
            Symbol::OBJECT(obj) => {
                if let Err(err) = write!(f, "{{ ") {
                    return Err(err);
                }
                for (name, symbol) in obj {
                    if let Err(err) = write!(f, "{} : {},", name, symbol) {
                        return Err(err);
                    }
                }
                write!(f, " }}")
            }
            Symbol::INT(val) => write!(f, "{}", val),
            Symbol::DOUBLE(val) => write!(f, "{}", val),
            Symbol::BOOL(val) => write!(f, "{}", val),
            Symbol::FUNC { return_type } => write!(f, "fn -> {}", return_type),
        }
    }
}

pub enum ValueType {
    LIST,
    OBJECT,
    INT,
    DOUBLE,
    BOOL,
    NULL,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ValueType::LIST => write!(f, "LIST"),
            ValueType::OBJECT => write!(f, "OBJECT"),
            ValueType::INT => write!(f, "INT"),
            ValueType::DOUBLE => write!(f, "DOUBLe"),
            ValueType::BOOL => write!(f, "BOOL"),
            ValueType::NULL => write!(f, "NULL"),
        }
    }
}
