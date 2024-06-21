//! Defines the IR object for a program.

use super::{expr::{Atom, Expr}, identifier::Identifier, Compilable};

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
    pub main: Vec<Statement>,
}

/// The main section of the program.
/// 
/// The main section is the entry point of the program. Currently a list of statements to be ran
pub struct Main {
    pub statements: Vec<Statement>,
}

/// A statement in the program.
/// 
/// Statements are used only inside blocks.
pub enum Statement {
    Expr(Expr),
    Bind{ident: Identifier, type_: Identifier, expr: Expr},
}

/// Stores the name of the atoms used in the program
/// 
/// The names are kebab-case strings without the atom prefix.
pub struct Atoms {
    pub atoms: Vec<String>,
}

impl Compilable for Program {
    fn compile(&self) -> String {
        let mut code = String::new();

        code.push_str(include_str!("core.h"));

        code.push_str("\n/* SECTION:: includes */\n");
        for include in &self.includes {
            code.push_str(&format!("#include \"{}\"\n", include));
        }

        // Compiles the program `atoms` section.
        code.push_str("\n/* SECTION:: atoms */\n");
        code.push_str(&self.atoms.compile());

        // Compiles the program `main` section.
        code.push_str("\n/* SECTION:: main */\n");
        code.push_str(&self.main.compile());
        
        code
    }

}

impl Compilable for Main {
    fn compile(&self) -> String {
        let mut code = String::new();

        code.push_str("int main() {\n");
        for expr in &self.statements {
            code.push_str(&format!("{};\n", expr.compile()));
        }
        code.push_str("return 0;\n");
        code.push_str("}\n");
        code
    }
}

impl Compilable for Statement {
    fn compile(&self) -> String {
        match self {
            Statement::Expr(expr) => expr.compile(),
            Statement::Bind{ident, type_, expr} => {
                format!("{} {} = {}",
                    type_.compile(),
                    ident.compile(),
                    expr.compile()
                )
            }
        }
    }
}

impl Compilable for Atoms {
    fn compile(&self) -> String {
        let mut code = String::new();

        for (i, atom) in self.atoms.iter().enumerate() {
            code.push_str(&format!("#define {} {}\n", atom, i));
        }

        code
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::expr::{Call, Literal};

    use super::*;

    #[test]
    fn test_program_compile() {
        let program = Program {
            includes: vec!["stdio.h".into()],
            
            atoms: vec!["a".into(), "b".into()],

            main: vec![
                Statement::Bind { ident: Identifier::Value("msg".into()), type_: Identifier::Type("String".into()), expr: Expr::Literal(Literal::String("Hello, World!".into())) },
                Statement::Expr(Expr::Call(Call {
                    symbol: "print".into(),
                    args: vec![
                        Expr::Identifier(Identifier::Value("msg".into()))
                    ]
                }))
            ],
        };

        println!("{}", program.compile());
    }
}