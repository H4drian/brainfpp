/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */
use crate::bfpp::debug::err;

use std::fs::File;
use std::io::{
    BufRead,
    BufReader
};

// for use in error and warning print in the left hand margins
fn get_spaces(x: usize) -> String {
    "".repeat(x.to_string().len())
}

fn read_file_line(filepath: String, line: usize) -> String {
    let file = File::open(filepath).expect("Unable to open file");
    let reader = BufReader::new(file);

    for (current_line, content) in reader.lines().enumerate() {
        if current_line == line {
            return content.expect("Unable to read line");
        }
    }

    String::new()
}

fn error_print(error: err::BfppError) {
    println!("error: {} [{}]", error.message, error.errcode);
    println!("{}|", get_spaces(error.line));
    println!("{}|{}", error.line, read_file_line(error.file, error.line));
    println!("{}|", get_spaces(error.line));
}

fn warning_print(warning: err::BfppWarning) {
    println!("warning: {}", warning.message);
    println!("{}|", get_spaces(warning.line));
    println!("{}|{}", warning.line, read_file_line(warning.file, warning.line));
    println!("{}|", get_spaces(warning.line));
}

pub fn print_all((errors, warnings): (Vec<err::BfppError>, Vec<err::BfppWarning>)) {
    for error in errors {
        error_print(error);
    }
    for warning in warnings {
        warning_print(warning);
    }
}