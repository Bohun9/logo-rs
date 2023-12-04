pub use super::super::parser::AstNode;
pub use super::Interpreter;

#[derive(Clone, Debug)]
pub struct LangFn {
    pub arity: usize,
    pub function: fn(&mut Interpreter, Vec<Value>) -> Value,
}

#[derive(Clone, Debug)]
pub struct UserFn {
    pub params: Vec<String>,
    pub body: AstNode,
}

impl UserFn {
    fn call(&self, args: Vec<Value>) {}
}

#[derive(Clone, Debug)]
pub enum LogoFn {
    LangFn(LangFn),
    UserFn(UserFn),
}

#[derive(Clone, Debug)]
pub enum Value {
    Nothing,
    Return,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<Value>),
    Function(LogoFn),
}
