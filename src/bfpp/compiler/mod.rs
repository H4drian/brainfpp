/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

use crate::bfpp::lexer;
use lexer::lexem::{
    Token,
    Lexem
};

mod optimizer;
use optimizer::optimize_bf;

pub fn compile_str_unoptimized(source_code: &str, linked_libs: Vec<&str>) -> String {
    let mut output_string: String = String::new();
    let lexems: Vec<Lexem> = lexer::lex_str(source_code, linked_libs);
    let mut data_ptr: usize = 0;

    for lexem in lexems {
        let instruction: Token = lexem.token;
        let arg: Option<usize> = lexem.arg;
        let line: usize = lexem.line;

        match instruction {
            Token::Sdp => {
                if arg == None {
                    panic!("Brainfpp: Error at line {line}: No data address given");
                }

                let offset: isize = (arg.unwrap() as isize) - (data_ptr as isize);

                if offset > 0 {
                    output_string.push_str(&">".repeat(offset as usize));
                    data_ptr += offset as usize;
                } else if offset < 0 {
                    output_string.push_str(&"<".repeat(offset as usize));
                    data_ptr -= offset as usize;
                } else {}
            }
            Token::Adp => output_string.push_str(&">".repeat(arg.unwrap())),
            Token::Sbp => output_string.push_str(&"<".repeat(arg.unwrap())),
            Token::Ldc => {
                if output_string.len() > 0 { output_string.push_str(&"[-]"); }
                output_string.push_str(&"+".repeat(arg.unwrap()));
            }
            Token::Add => output_string.push_str(&"+".repeat(arg.unwrap())),
            Token::Sub => output_string.push_str(&"-".repeat(arg.unwrap())),
            Token::Out => output_string.push_str(&"."),
            Token::Inn => output_string.push_str(&","),
            Token::Slp => output_string.push_str(&"["),
            Token::Elp => output_string.push_str(&"]"),
            Token::Noop => {}
            // subroutines should be delt with by the lexer
            Token::Def => {}
            Token::Edef => {}
            Token::Call => {}
        }
    }

    output_string
}

pub fn compile_str_optimized(source_code: &str, linked_libs: Vec<&str>) -> String {
    optimize_bf(&compile_str_unoptimized(source_code, linked_libs))
}

pub fn lex_str_to_string(source_code: &str, linked_libs: Vec<&str>) -> String {
    let mut output_string: String = String::new();

    for lexem in lexer::lex_str(source_code, linked_libs) {
        output_string.push_str(format!("{:?}\n", lexem).as_str());
    }

    output_string
}
