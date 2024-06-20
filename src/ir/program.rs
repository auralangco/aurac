//! Defines the IR object for a program.

use super::{Atom, Compilable, Literal};

/// The IR object for a program. It represents a correct Aura program (syntax and semantics).
/// 
/// The program should contain enough information to produce a correct C program.
pub struct Program {
    /// The list of C header files to include in the compilation.
    pub includes: Vec<String>,

    /// Atoms are defined globally in the program.
    /// A total of 2^64 atoms can be defined in a program.
    pub atoms: Vec<Atom>,

    /// The main program.
    pub program: Vec<Literal>,
}

impl Compilable for Program {
    fn compile(&self) -> String {
        let mut code = String::new();

        code.push_str(include_str!("core.h"));

        code.push_str("\n// Section includes\n");
        for include in &self.includes {
            code.push_str(&format!("#include \"{}\"\n", include));
        }

        code.push_str("\n// Section atoms\n");
        for (i, atom) in self.atoms.iter().enumerate() {
            code.push_str(&format!("#define {} {}\n", atom.repr, i));
        }

        code.push_str("\n// Section main\n");
        code.push_str("int main() {\n");
        for literal in &self.program {
            code.push_str(&format!("{};\n", literal.compile()));
        }
        code.push_str("return 0;\n");
        code.push_str("}\n");
        code
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_compile() {
        let program = Program {
            includes: vec!["stdio.h".into()],
            atoms: vec![Atom::from_identifier("a"), Atom::from_identifier("b")],
            program: vec![
                Literal::Int(42),
                Literal::Float(3.14),
                Literal::String("hello".into()),
                Literal::Bool(true),
                Literal::Bool(false),
                Literal::Atom(Atom::from_identifier("a")),
                Literal::Atom(Atom::from_identifier("b")),
            ],
        };

        println!("{}", program.compile());
    }
}