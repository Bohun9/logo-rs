pub use super::super::parser::AstNode;
pub use super::Interpreter;

#[derive(Clone, Debug)]
pub enum LogoFn {
    LangFn {
        arity: usize,
        function: fn(&mut Interpreter, Vec<Value>) -> Value,
    },
    UserFn {
        params: Vec<String>,
        body: AstNode,
    },
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
