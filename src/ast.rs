#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Bool,
    StringType,
}

#[derive(Debug, Clone)]
pub struct Expression(Vec<ExpressionComponent>);

#[derive(Debug, Clone)]
pub enum ExpressionComponent {
    Number(Type, f64),
    StringLit(String),
    Char(char),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Variable),
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub var_type: Type,
    pub name: String,
    pub value: Expression,
}

impl Variable {
    pub fn new(var_type: Type, name: String, value: Expression) -> Self {
        Self {
            var_type,
            name,
            value,
        }
    }
}
