use super::Compilable;

pub enum Identifier {
    Value(String),
    Type(String),
}

impl Compilable for Identifier {
    fn compile(&self) -> String {
        match self {
            Identifier::Value(name) => name.clone(),
            Identifier::Type(name) => name.clone(),
        }
    }
}