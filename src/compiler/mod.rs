use std::collections::HashMap;

use crate::{
    parser::{Expr, Program, Statement},
    vm::Instruction,
};

pub struct Compiler {
    var_stack: Vec<String>,
    bindings: HashMap<String, Vec<Instruction>>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            var_stack: vec![],
            bindings: HashMap::new(),
        }
    }

    pub fn compile(&mut self, program: Program) -> Result<Vec<Instruction>, String> {
        for statement in program.statements {
            self.compile_statement(statement)?;
        }

        if let Some(main_expr) = program.main_expr {
            self.compile_expr(&main_expr)
        } else {
            Ok(vec![Instruction::Lam(vec![Instruction::Var(0)])])
        }
    }

    fn compile_statement(&mut self, statement: Statement) -> Result<(), String> {
        match statement {
            Statement::LetBinding { name, value } => {
                let compiled_value = self.compile_expr(&value)?;
                self.bindings.insert(name, compiled_value);
                Ok(())
            }
        }
    }

    pub fn compile_expr(&mut self, expr: &Expr) -> Result<Vec<Instruction>, String> {
        match expr {
            Expr::Var(name) => {
                if let Some(instructions) = self.bindings.get(name) {
                    return Ok(instructions.clone());
                }

                for (i, var_name) in self.var_stack.iter().rev().enumerate() {
                    if var_name == name {
                        return Ok(vec![Instruction::Var(i)]);
                    }
                }

                Err(format!("Unbound variable: {}", name))
            }

            Expr::Lambda { param, body } => {
                self.var_stack.push(param.clone());

                let body_instructions = self.compile_expr(body)?;

                self.var_stack.pop();

                Ok(vec![Instruction::Lam(body_instructions)])
            }

            Expr::App { func, arg } => {
                let mut instructions = Vec::new();

                let func_instructions = self.compile_expr(func)?;
                let arg_instructions = self.compile_expr(arg)?;

                // Push function, then argument, then Apply
                instructions.extend(func_instructions);
                instructions.extend(arg_instructions);
                instructions.push(Instruction::App);

                Ok(instructions)
            }

            Expr::Builtin(name) => self.compile_builtin(name),
        }
    }

    fn compile_builtin(&self, name: &str) -> Result<Vec<Instruction>, String> {
        match name {
            "SUCC" => Ok(vec![Instruction::Lam(vec![
                // λn.
                Instruction::Lam(vec![
                    // λf.
                    Instruction::Lam(vec![
                        // λx.
                        Instruction::Var(1), // f
                        Instruction::Var(2), // n
                        Instruction::Var(1), // f
                        Instruction::Var(0), // x
                        Instruction::App,    // f x
                        Instruction::App,    // n f x
                        Instruction::App,    // f (n f x)
                    ]),
                ]),
            ])]),

            "ADD" => Ok(vec![Instruction::Lam(vec![
                // λm.
                Instruction::Lam(vec![
                    // λn.
                    Instruction::Lam(vec![
                        // λf.
                        Instruction::Lam(vec![
                            // λx.
                            Instruction::Var(3), // m
                            Instruction::Var(1), // f
                            Instruction::Var(2), // n
                            Instruction::Var(1), // f
                            Instruction::Var(0), // x
                            Instruction::App,    // f x
                            Instruction::App,    // n f x
                            Instruction::App,    // m f (n f x)
                        ]),
                    ]),
                ]),
            ])]),

            "TRUE" => Ok(vec![Instruction::Lam(vec![Instruction::Lam(vec![
                Instruction::Var(1),
            ])])]),

            "FALSE" => Ok(vec![Instruction::Lam(vec![Instruction::Lam(vec![
                Instruction::Var(0),
            ])])]),

            "AS_NAT" | "AS_BOOL" | "AS_LIST" => {
                Ok(vec![Instruction::Lam(vec![Instruction::Var(0)])])
            }

            _ => Err(format!("Unknown builtin: {}", name)),
        }
    }
}
