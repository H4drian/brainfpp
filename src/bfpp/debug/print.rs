/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */
use crate::bfpp::debug::err;

// for use in error and warning print in the left hand margins
fn get_spaces(x: usize) -> String {
    "".repeat(x.to_string().len())
}

fn error_print(error: err::BfppError) {
    println!("error: {} [{}]", error.message, error.errcode);
    println!("{}|", get_spaces(error.line));
    println!("{}|", error.line);
}

fn warning_print(warning: err::BfppWarning) {}