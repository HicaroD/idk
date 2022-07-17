pub mod c;

use crate::ast::Ast;
use crate::lexer::Token;
use crate::parser::Expression;

// TODO: The result of an expression is not always an f64
//pub fn evaluate_ast(expression: Expression) -> Result<f64, String> {
//    match expression {
//        Expression::Int(value) => Ok(f64::from(value)),
//
//        Expression::Float(value) => Ok(value),
//
//        Expression::BinaryExpr(lhs, operation, rhs) => {
//            let left = evaluate_ast(*lhs)?;
//            let right = evaluate_ast(*rhs)?;
//
//            match operation {
//                Token::Plus => Ok(left + right),
//                Token::Minus => Ok(left - right),
//                Token::Times => Ok(left * right),
//                Token::Divides => Ok(left / right),
//                _ => Err(format!(
//                    "Operator not implemented or invalid: {:?}",
//                    operation
//                )),
//            }
//        }
//
//        _ => Err(format!("Expression not implemented: {:?}", expression)),
//    }
//}

// TODO: Stack evaluation for RPN expression
pub fn evaluate_rpn_expression(expression: Expression) -> Result<f64, String> {
    Ok(10.0)
}

pub trait CodeGenerator {
    fn generate(&mut self, ast: Vec<Ast>) -> Result<(), String>;
}
