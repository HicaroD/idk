#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    Bool,
}

#[derive(Debug)]
pub enum Expression {
    Number(f64),
}

#[derive(Debug)]
pub enum Statement {
    Assignment(Variable),
}

#[derive(Debug)]
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
