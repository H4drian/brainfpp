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

impl Token {
    pub fn takes_arg(self) -> bool {
       !(  self == Token::Slp 
        || self == Token::Elp 
        || self == Token::Edef
        || self == Token::Out
        || self == Token::Inn
        || self == Token::Noop) 
    }

    pub fn alters_pointer(self) -> bool {
        self == Token::Sdp || self == Token::Adp || self == Token::Sbp
    }

    pub fn alters_data(self) -> bool {
        self == Token::Ldc || self == Token::Add || self == Token::Sub
    }
}

#[derive(Debug, Clone)]
pub struct Lexem {
    pub token: Token,
    pub arg: Option<usize>,
    pub line: usize
}

pub struct Subroutine {
    pub name: String,
    pub code: Vec<Lexem>
}

pub trait New {
    fn new() -> Self;
}

impl New for Lexem {
    fn new() -> Lexem {
        Lexem {
            token: Token::Noop,
            arg: None,
            line: 0
        } 
    }
}

impl New for Subroutine {
    fn new() -> Subroutine {
        Subroutine {
            name: String::new(),
            code: Vec::new()
        }
    }
}