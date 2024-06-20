//! Definitions for the Aura Intermediate Representation (IR).

/// A trait implemented by IR objects to compile to a given target language.
pub trait Compilable {
    // TODO: The target language as a type parameter?

    /// Compile the IR object to a target language.
    /// returns: The compiled code as a string.
    fn compile(&self) -> String;
}

pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
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

mod tests {
    use super::*;

    #[test]
    fn test_literal_compile() {
        assert_eq!(Literal::Int(42).compile(), "42");
        assert_eq!(Literal::Float(3.14).compile(), "3.14");
        assert_eq!(Literal::String("hello".into()).compile(), "\"hello\"");
        assert_eq!(Literal::Bool(true).compile(), "true");
    }
}