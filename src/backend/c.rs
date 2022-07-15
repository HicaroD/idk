use crate::ast::*;
use std::fs::File;
use std::io::Write;

pub struct C {
    ast: Vec<Ast>,
}

impl C {
    pub fn new(ast: Vec<Ast>) -> Self {
        Self { ast }
    }

    pub fn get_c_type(&self, type_: &Type) -> Result<&str, String> {
        match type_ {
            Type::Int => Ok("int"),
            Type::Float => Ok("float"),
            Type::Bool => Ok("bool"),
            Type::StringType => Ok("char[]"),
            _ => Err("Cannot get parameter type".to_string()),
        }
    }

    pub fn get_function_parameters(&self, parameters: &Vec<Parameter>) -> Result<String, String> {
        let mut c_parameters = String::new();

        for parameter in parameters {
            let parameter_type = self.get_c_type(&parameter.parameter_type)?;
            c_parameters += &format!("{} {},", parameter_type, parameter.name);
        }

        Ok(c_parameters)
    }

    pub fn build_c_function(&self, function_node: Function) -> Result<String, String> {
        let parameters = self.get_function_parameters(&function_node.parameters)?;
        let return_type = match function_node.return_type {
            Some(t) => self.get_c_type(&t)?,
            None => "void",
        };
        Ok(format!(
            "{} {}({}) {{{}}}",
            return_type, function_node.name, parameters, "statement here"
        ))
    }

    pub fn generate_c_code(&self) -> Result<(), String> {
        // TODO: Avoid unwrap
        let mut c_code = File::create("code.c").unwrap();

        for ast_node in self.ast.iter() {
            match ast_node {
                Ast::Function(function) => {
                    c_code.write_all(self.build_c_function(function.clone())?.as_bytes());
                }

                _ => return Err("Invalid AST node".to_string()),
            }
        }

        Ok(())
    }
}
