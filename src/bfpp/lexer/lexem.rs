/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

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

#[derive(Debug, Clone)]
pub struct Lexem {
    pub token: Token,
    pub arg: Option<usize>,
    pub line: usize
}

pub struct Subroutine {
    name: String,
    code: Vec<Lexem>
}