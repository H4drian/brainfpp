/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

use crate::bfpp::utils::BaseTrait;

/// The token type for the lexer. Each possible instruction has a token.
#[derive(Debug, Clone)]
pub enum Token {
    Sdp,        // set data pointer
    Adp,        // add data pointer
    Sbp,        // sub pointer
    Ldc,        // load cell
    Add,        // add data
    Sub,        // sub from data
    Out,        // output 
    Inn,        // input
    Slp,        // start loop
    Elp,        // end loop
    Def,        // define subroutine
    Edef,       // End definition
    Call,       // call subroutine
    Noop        // no op
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Sdp, Token::Sdp) => true,
            (Token::Adp, Token::Adp) => true,
            (Token::Sbp, Token::Sbp) => true,
            (Token::Ldc, Token::Ldc) => true,
            (Token::Add, Token::Add) => true,
            (Token::Sub, Token::Sub) => true,
            (Token::Out, Token::Out) => true,
            (Token::Inn, Token::Inn) => true,
            (Token::Slp, Token::Slp) => true,
            (Token::Elp, Token::Elp) => true,
            (Token::Def, Token::Def) => true,
            (Token::Edef, Token::Edef) => true,
            (Token::Call, Token::Call) => true,
            (Token::Noop, Token::Noop) => true,
            _ => false,
        }
    }
}

/// The lexem type for the lexer. Each lexem holds an instruction token, argument, and line number.
#[derive(Debug, Clone)]
pub struct Lexem {
    pub token: Token,
    pub arg: Option<usize>,
    pub line: usize
}

impl BaseTrait for Lexem {
    fn new() -> Lexem {
        Lexem {
            token: Token::Noop,
            arg: None,
            line: 0
        }
    }

    fn reset(&mut self) -> () {
        self.token = Token::Noop;
        self.arg = None;
        self.line = 0;
    }
}

/// The subroutine type for the lexer. Each subroutine holds a name and code.
pub struct Subroutine {
    pub name: String,
    pub code: Vec<Lexem>
}

impl BaseTrait for Subroutine {
    fn new() -> Subroutine {
        Subroutine {
            name: String::new(),
            code: Vec::new(),
        }
    }

    fn reset(&mut self) -> () {
        self.name = String::new();
        self.code = Vec::new();
    }
}