//! Definitions for the Aura Intermediate Representation (IR).

mod program;

/// A trait implemented by IR objects to compile to a given target language.
trait Compilable {
    // TODO: The target language as a type parameter?

    /// Compile the IR object to a target language.
    /// returns: The compiled code as a string.
    fn compile(&self) -> String;
}

/// The IR object for a primitive literal value.
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Atom(Atom),
}

pub struct Atom {
    pub repr: String,
}

impl Compilable for Literal {
    fn compile(&self) -> String {
        match self {
            Literal::Int(i) => i.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Bool(b) => b.to_string(),
            Literal::Atom(atom) => atom.repr.clone(),
        }
    }
}

impl Atom {
    fn from_identifier(id: &str) -> Self {
        Self { repr: format!("__Atom_{}", id.to_uppercase().replace("-", "_")) }
    }

    fn into_identifier(&self) -> String {
        self.repr.trim_start_matches("__Atom_").to_lowercase().replace("_", "-")
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
        assert_eq!(Literal::Atom(Atom::from_identifier("a")).compile(), "__Atom_A");
    }
}