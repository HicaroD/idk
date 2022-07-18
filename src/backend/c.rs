use crate::ast::*;
use crate::backend::{evaluate_ast, CodeGenerator};
use std::fs::File;
use std::io::Write;

pub struct C {
    source_code: File,
}

impl CodeGenerator for C {
    fn generate(&mut self, ast: Vec<Ast>) -> Result<(), String> {
        self.generate_c_code(ast)
    }
}

impl C {
    pub fn new() -> Self {
        Self {
            // FIXME: Avoid unwrap
            source_code: File::create("code.c").unwrap(),
        }
    }

    fn setup_code(&mut self) {
        let libraries = vec!["stdio.h", "stdlib.h"];
        for library in libraries.iter() {
            self.source_code
                .write_all(format!("#include \"{}\"\n", library).as_bytes());
        }
        self.source_code.write_all("\n".as_bytes());
    }

    fn get_c_type(&self, type_: &Type) -> Result<&str, String> {
        match type_ {
            Type::Int => Ok("int"),
            Type::Float => Ok("float"),
            Type::Bool => Ok("bool"),
            Type::StringType => Ok("char[]"),
            _ => Err("Can't get parameter type".to_string()),
        }
    }

    fn get_function_parameters(&self, parameters: &Vec<Parameter>) -> Result<String, String> {
        let mut c_parameters = String::new();

        for parameter in parameters {
            let parameter_type = self.get_c_type(&parameter.parameter_type)?;
            c_parameters += &format!("{} {}, ", parameter_type, parameter.name);
        }
        c_parameters = c_parameters.trim_end_matches(", ").to_string();
        Ok(c_parameters)
    }

    fn build_c_block(&self, block: Block) -> Result<String, String> {
        let mut statements = String::new();

        for statement in block.itens.iter() {
            match statement {
                Ast::Assignment(assignment) => {
                    statements += &self.build_c_assignment(assignment.clone())?
                }
                _ => return Err("Unable to generate C block".to_string()),
            };
        }
        Ok(statements)
    }

    pub fn build_c_assignment(&self, assignment: Assignment) -> Result<String, String> {
        let var_type = self.get_c_type(&assignment.var_type)?;
        let name = assignment.name;
        let value = evaluate_ast(assignment.value)?.to_string();

        Ok(format!("\t{} {} = {};\n", var_type, name, value))
    }

    fn build_c_function(&self, function_node: Function) -> Result<String, String> {
        let parameters = self.get_function_parameters(&function_node.parameters)?;
        let return_type = match function_node.return_type {
            Some(t) => self.get_c_type(&t)?,
            None => "void",
        };

        let block = self.build_c_block(function_node.body)?;

        Ok(format!(
            "{} {}({}) {{\n{}}}",
            return_type, function_node.name, parameters, block
        ))
    }

    pub fn generate_c_code(&mut self, ast: Vec<Ast>) -> Result<(), String> {
        self.setup_code();

        for node in ast.iter() {
            match node {
                Ast::Function(function) => {
                    self.source_code
                        .write_all(self.build_c_function(function.clone())?.as_bytes());
                }

                _ => return Err("Unable to generate C code".to_string()),
            }
        }

        Ok(())
    }
}
