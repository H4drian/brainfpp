mod bfpp;

use std::env;
use std::fs;
use std::io::Write;

fn main() {
    let command: Option<String> = env::args().nth(1);

    match command.unwrap_or("".to_string()).as_str() {
        "compile" => {
            let source_file: Option<String> = env::args().nth(2);
            let out_file: Option<String> = env::args().nth(3);
            let flag: Option<String> = env::args().nth(4);

            if source_file.is_none() {
                panic!("Brainfpp: No source file given.");
            }

            let source_code = match fs::read_to_string(source_file.unwrap()) {
                Ok(code) => code,
                Err(_) => panic!("Brainfpp: Error reading source file.")
            };

            let compiled_code = match flag.as_deref() {
                Some("-r") | Some("-release") => bfpp::compiler::compile_str_optimized(&source_code),
                Some("-d") | Some("-dev") => bfpp::compiler::compile_str_unoptimized(&source_code),
                _ => bfpp::compiler::compile_str_unoptimized(&source_code)
            };

            if out_file.is_none() {
                println!("{}", compiled_code);
            } else {
                let out_file_name = out_file.unwrap_or("result".to_string());
                let mut outfile_obj = match fs::File::create(&out_file_name) {
                    Ok(file) => file,
                    Err(_) => panic!("Brainfpp: Error compiling brainfuck program")
                };

                if let Err(err) = outfile_obj.write_all(compiled_code.as_bytes()) {
                    panic!("Brainfpp: Error writing to brainfuck program: {:?}", err);
                }
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

        _ => panic!("Brainfpp: No command arguments given.")
    }
}