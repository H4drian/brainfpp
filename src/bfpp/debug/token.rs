use crate::bfpp::lexer::lexem::Token;

pub struct DebugToken {
    pub token: Token,
    pub arg: Option<usize>,
    pub string_arg: Option<String>,
    pub line: usize,
    pub code: String
}