use super::{identifier::Identifier, Compilable};

pub enum Expr {
    Literal(Literal),
    Call(Call),
    Atom(Atom),
    Identifier(Identifier),
}

/// The IR object for a primitive literal value.
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

pub struct Call {
    pub symbol: String,
    pub args: Vec<Expr>,
}

pub struct Atom(String);

impl Compilable for Expr {
    fn compile(&self) -> String {
        match self {
            Expr::Literal(literal) => literal.compile(),
            Expr::Call(call) => call.compile(),
            Expr::Atom(atom) => atom.compile(),
            Expr::Identifier(identifier) => identifier.compile(),
        }
    }
}

impl Compilable for Literal {
    fn compile(&self) -> String {
        match self {
            Literal::Int(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Bool(b) => b.to_string(),
        }
    }
}

impl Compilable for Call {
    fn compile(&self) -> String {
        let args = self.args.iter().map(|arg| arg.compile()).collect::<Vec<String>>().join(", ");
        format!("{}({})", self.symbol, args)
    }
}

impl Atom {
    fn from_identifier(id: &str) -> Self {
        Self(format!("__Atom_{}", id.to_uppercase().replace("-", "_")))
    }

    fn into_identifier(&self) -> String {
        self.0.trim_start_matches("__Atom_").to_lowercase().replace("_", "-")
    }
}

impl From<&str> for Atom {
    fn from(id: &str) -> Self {
        Atom::from_identifier(id)
    }
}

impl Into<String> for Atom {
    fn into(self) -> String {
        self.into_identifier()
    }
}

impl Compilable for Atom {
    fn compile(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_compile() {
        assert_eq!(Literal::Int(42).compile(), "42");
        assert_eq!(Literal::Float(3.14).compile(), "3.14");
        assert_eq!(Literal::String("hello".into()).compile(), "\"hello\"");
        assert_eq!(Literal::Bool(true).compile(), "true");
        assert_eq!(Literal::Bool(false).compile(), "false");
    }
}