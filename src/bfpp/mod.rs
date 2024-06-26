/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

pub mod interpreter;
pub mod compiler;
pub mod cli;
pub mod lexer;
mod utils;

pub fn get_stdlib() -> String {
    include_str!("std.bfpp").to_string()
}