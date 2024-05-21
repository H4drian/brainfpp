/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */

pub mod print;
mod err;
mod token;

use token::DebugToken;
use crate::bfpp::lexer::lexem::Token;
use err::{
    BfppError,
    BfppWarning
};

trait StringDebug {
    fn is_variable_case(&self) -> bool;
    fn has_upper_case(&self) -> bool;
    fn has_lower_case(&self) -> bool;
}

impl StringDebug for String {
    fn is_variable_case(&self) -> bool {
        self.chars().all(|c| c.is_ascii_alphabetic() || c == '_')
    }

    fn has_upper_case(&self) -> bool {
        
        self.chars().any(|c| c.is_ascii_uppercase())
    }

    fn has_lower_case(&self) -> bool {
        self.chars().any(|c| c.is_ascii_lowercase())
    }
}

fn get_debug_token(source_code: String) -> Vec<DebugToken> {
    let mut tokens: Vec<DebugToken> = Vec::new();
    let lines: Vec<&str> = source_code.lines().collect();
    let mut line_count: usize = 0;

    for line in lines {
        let mut dtoken: DebugToken = DebugToken::new();
        
        line_count += 1;
        dtoken.line = line_count;
        dtoken.code = line.to_string();

        dtoken.token = match line.split_whitespace().next().unwrap_or("").to_lowercase().as_str() {
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

        dtoken.string_arg = match line.split_whitespace().nth(1) {
            Some(s) => Some(s.to_string()),
            None => None
        };

        dtoken.arg = match dtoken.string_arg.clone() {
            Some(s) => Some(s.parse::<usize>().unwrap()),
            None => None
        };

        tokens.push(dtoken); 
    }

    tokens
}

pub fn debug_source_code(source_code: String, start: usize, end: usize, filepath: String) -> (Vec<BfppError>, Vec<BfppWarning>) {
    let mut errors: Vec<BfppError> = Vec::new();
    let mut warnings: Vec<BfppWarning> = Vec::new();
    let tokens: Vec<DebugToken> = get_debug_token(source_code);

    let mut uses_upper_case: bool = false;

    for dtoken in tokens {
        if dtoken.line < start || dtoken.line > end {
            continue;
        }

        if dtoken.code.has_upper_case() {
            uses_upper_case = true;
        }
        // TODO: add --ignore-case to the debuger
        if dtoken.code.has_lower_case() && uses_upper_case {
            warnings.push(BfppWarning {
                message: "program uses both lowercase and uppercase instructions. If this is intentional use --ignore-case".to_string(),
                file: filepath.to_string(),
                line: dtoken.line
            })
        }
            
        let token_string: String = dtoken.code.split_whitespace().next().unwrap_or("").to_string();

        if token_string.is_variable_case() {
            warnings.push(BfppWarning {
                message: "instructions should be lowercase or uppercase".to_string(),
                file: filepath.clone(),
                line: dtoken.line
            });
        }

        if dtoken.token.clone().takes_arg() {
            if dtoken.arg.is_none() {
                errors.push(BfppError {
                    message: "argument taking instruction has no argument".to_string(),
                    errcode: 1,
                    file: filepath.clone(),
                    line: dtoken.line
                });
            }

            if dtoken.token.clone().alters_data() {
                if dtoken.arg.unwrap_or(0) > 255 {
                    errors.push(BfppError {
                        message: "argument is larger than the max for unsigned 8 bit values and will likely cause problems when running the resulting bf program. Try a smaller value".to_string(),
                        errcode: 2,
                        file: filepath.clone(),
                        line: dtoken.line
                    });
                }
            }

            if dtoken.token.clone().alters_pointer() {
                if dtoken.arg.unwrap_or(0) > 30000 {
                    warnings.push(BfppWarning {
                        message: "argument is larger than the initial cell count of 30,000. This could cause errors depending on if the bf interpreter automatically resizes the data tape.".to_string(),
                        file: filepath.clone(),
                        line: dtoken.line
                    });
                }
            }
            
        } else {
            if dtoken.arg.is_some() {
                errors.push(BfppError {
                    message: "non-argument taking instruction has argument".to_string(),
                    errcode: 2,
                    file: filepath.clone(),
                    line: dtoken.line
                })
            }
        }
    }
    
    (errors, warnings)
}