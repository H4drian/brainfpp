mod bfpp;

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let command: Option<String> = env::args().nth(1);

    match command.unwrap_or("".to_string()).as_str() {
        "compile" => {
            let source_file: Option<String> = env::args().nth(2);
            let mut out_file: Option<String> = env::args().nth(3);
            let mut flag_arg: Option<String> = env::args().nth(4);

            if source_file.is_none() {
                panic!("Brainfpp: No source file given.");
            }

            let source_code = match fs::read_to_string(source_file.unwrap()) {
                Ok(code) => code,
                Err(_) => panic!("Brainfpp: Error reading source file.")
            };

            if let Some(out) = out_file.clone() {
                // Check if out_file starts with '-'
                if out.starts_with('-') {
                    flag_arg = out_file;
                    out_file = None;
                }
            }

            let compiled_code = match flag_arg.as_deref() {
                Some(arg) if arg.starts_with('-') => {
                    match arg {
                        "-r" | "-release" => bfpp::compiler::compile_str_optimized(&source_code),
                        "-d" | "-dev" => bfpp::compiler::compile_str_unoptimized(&source_code),
                        _ => bfpp::compiler::compile_str_unoptimized(&source_code)
                    }
                }
                _ => bfpp::compiler::compile_str_unoptimized(&source_code)
            };

            if let Some(out) = out_file {
                let out_file_name = out;
                let mut outfile_obj = match fs::File::create(&out_file_name) {
                    Ok(file) => file,
                    Err(_) => panic!("Brainfpp: Error compiling brainfuck program")
                };

                if let Err(err) = outfile_obj.write_all(compiled_code.as_bytes()) {
                    panic!("Brainfpp: Error writing to brainfuck program: {:?}", err);
                }
            } else {
                println!("{}", compiled_code);
            }
        }

        "run" => {
            let file: Option<String> = env::args().nth(2);

            match file {
                Some(filepath) => {
                    let code = match fs::read_to_string(&filepath) {
                        Ok(code) => code,
                        Err(_) => panic!("Brainfpp: Error reading file.")
                    };

                    bfpp::interpreter::interpret_str(&code);
                }
                None => panic!("Brainfpp: No brainfuck file given.")
            }
        }

        "lex" => {
            let source_file: Option<String> = env::args().nth(2);
            let out_file: Option<String> = env::args().nth(3);

            if source_file.is_none() {
                panic!("Brainfpp: No source file given.");
            }

            let source_code: String = match fs::read_to_string(source_file.unwrap()) {
                Ok(code) => code,
                Err(_) => panic!("Brainfpp: Error reading file.")
            };
            
            let lexed_code: String = bfpp::compiler::lex_str_to_string(&source_code);

            if out_file.is_none() {
                println!("{lexed_code}");
            } else {
                let out_file_name = out_file.unwrap();
                let mut outfile_obj = match fs::File::create(&out_file_name) {
                    Ok(file) => file,
                    Err(err) => panic!("Brainfpp: Error creating lexdump file: {:?}", err)
                };

                if let Err(err) = outfile_obj.write_all(lexed_code.as_bytes()) {
                    panic!("Brainfpp: Error writing to lexdump file: {:?}", err);
                }
            }
        }

        _ => panic!("Brainfpp: No command arguments given.")
    }
}