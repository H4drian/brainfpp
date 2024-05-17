/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

mod lexem;
use lexem::*;

pub fn lex_str(source_code: &str) -> Vec<Lexem> {
    let mut lexem_vec: Vec<Lexem> = Vec::new();
    let mut subroutine_vec: Vec<Subroutine> = Vec::new();
    let mut in_subroutine_definition = false;
    let mut current_subroutine_name = String::new();
    let mut current_subroutine_code: Vec<Lexem> = Vec::new();

    let lines: Vec<&str> = source_code.lines().collect();
    let mut line_count: usize = 0;

    for line in lines {
        line_count += 1;

        let instruction: Token = match line.split_whitespace().next().unwrap_or("").to_lowercase().as_str() {
            "sdp" => Token::Sdp,
            "adp" => Token::Adp,
            "sbp" => Token::Sbp,
            "ldc" => Token::Ldc,
            "add" => Token::Add,
            "sub" => Token::Sub,
            "out" => Token::Out,
            "inn" => Token::Inn,
            "["   => Token::Slp,
            "]"   => Token::Elp,
            "def" => Token::Def,
            "edef"=> Token::Edef, 
            "call"=> Token::Call,
            _     => Token::Noop, 
        };
        let instruction_arg: Option<usize> = line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok());

        if in_subroutine_definition {
            if instruction == Token::Edef {
                subroutine_vec.push(Subroutine {
                    name: current_subroutine_name.clone(),
                    code: current_subroutine_code.clone(),
                });
                in_subroutine_definition = false;
                current_subroutine_name.clear();
                current_subroutine_code.clear();
            } else {
                current_subroutine_code.push(Lexem {
                    token: instruction,
                    arg: instruction_arg,
                    line: line_count
                });
            }
        } else {
            if instruction == Token::Def {
                in_subroutine_definition = true;
                if let Some(name) = line.split_whitespace().nth(1) {
                    current_subroutine_name = name.to_string();
                }
            } else if instruction == Token::Call {
                if let Some(subroutine) = subroutine_vec.iter().find(|sub| sub.name == line.split_whitespace().nth(1).unwrap_or("")) {
                    lexem_vec.extend_from_slice(&subroutine.code);
                } else {
                    // Handle error: subroutine not found
                    panic!("Subroutine not found: {:?}", line.split_whitespace().nth(1));
                }
            } else {
                lexem_vec.push(Lexem {
                    token: instruction,
                    arg: instruction_arg,
                    line: line_count
                });
            }
        }
    }

    lexem_vec
}
