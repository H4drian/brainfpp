/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */

use crate::bfpp::lexer::lexem::Token;

pub struct DebugToken {
    pub token: Token,
    pub arg: Option<usize>,
    pub string_arg: Option<String>,
    pub line: usize,
    pub code: String
}

impl DebugToken {
    pub fn new() -> DebugToken {
        DebugToken {
            token: Token::Noop,
            arg: None,
            string_arg: None,
            line: 0,
            code: String::new()
        }
    }
}