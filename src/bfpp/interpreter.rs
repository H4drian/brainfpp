/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

// bfpp built-in brainfuck interpreter

pub fn interpret_str(code: &str) -> () { 
    let code_chars: Vec<char> = code.chars().collect();

    let mut tape: Vec<u8> = vec![0; 30000];
    let mut data_ptr: usize = 0;
    let mut instruction_ptr: usize = 0;

    while instruction_ptr < code_chars.len() {
        match code_chars.get(instruction_ptr) {
            Some(c) => match c {
                '>' => {
                    data_ptr += 1;
                    if data_ptr > tape.len() {
                        tape.push(0);
                    }
                }
                '<' => data_ptr -= 1,
                '+' => {
                    if tape[data_ptr] == 255 {
                        tape[data_ptr] = 0;
                    } else {
                        tape[data_ptr] += 1;
                    }
                }
                '-' => tape[data_ptr] -= 1,
                '.' => print!("{}", tape[data_ptr] as char),
                ',' => {
                    
                }
                '[' => {
                    if tape[data_ptr] == 0 {
                        let mut bracket_count: usize = 1;
                        while bracket_count > 0 {
                            instruction_ptr += 1;
                            if code_chars.get(instruction_ptr) == Some(&'[') {
                                bracket_count += 1;
                            } else if code_chars.get(instruction_ptr) == Some(&']') {
                                bracket_count -= 1;
                            } 
                        }
                    }
                }
                ']' => {
                    if tape[data_ptr] != 0 {
                        let mut bracket_count: usize = 1;
                        while bracket_count > 0 {
                            instruction_ptr -= 1;
                            if code_chars.get(instruction_ptr) == Some(&']') {
                                bracket_count += 1;
                            } else if code_chars.get(instruction_ptr) == Some(&'[') {
                                bracket_count -= 1;
                            } 
                        }
                    }
                }
                 _  => {}
            }
            None => {}
        }
        instruction_ptr += 1;
    }
}
