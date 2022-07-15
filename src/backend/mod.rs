pub mod c;

pub trait CodeGenerator {
    fn generate(&self) -> Result<String, String>;
}
