/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

mod bfpp;

use std::fs::{
    self,
    OpenOptions
};
use std::io::Write;

extern crate clap;
use clap::ArgMatches;

fn write_output_file(filepath: String, content: String) {
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(filepath) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("brainfpp: Error opening outfile, {}", err);
                return;
            }
        };
    
    match file.write_all(content.as_bytes()) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("brainfpp: Error writing to outfile, {}", err);
            return;
        }
    }

    if let Err(err) = file.flush() {
        eprintln!("brainfpp: Error flushing file buffer: {}", err);
    }
}

fn read_input_file(filepath: String) -> String {
    match fs::read_to_string(filepath) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("brainfpp: Error reading from input file, {}", err);
            String::new()
        }
    }
}

fn main() {
    let matches: ArgMatches = bfpp::cli::get_matches();

    match matches.subcommand() {
        Some(("compile", compile_m)) => {
            let mut infile: String = String::new();

            if let Some(i) = compile_m.value_of("infile") {
                infile = i.to_string();
            } 
            if let Some(o) = compile_m.value_of("outfile") {
                let outfile: String = o.to_string();
                let infile_content: String = read_input_file(infile);

                if compile_m.is_present("release") {
                    let bf_code: String = bfpp::compiler::compile_str_optimized(&infile_content);
                    write_output_file(outfile, bf_code);
                } else {
                    let bf_code: String = bfpp::compiler::compile_str_unoptimized(&infile_content);
                    write_output_file(outfile, bf_code);
                }
            } else {
                let mut infile_content: String = String::new();
                    match fs::read_to_string(infile) {
                        Ok(content) => infile_content = content,
                        Err(err) => eprintln!("brainfpp: Error reading from input file, {}", err)
                    }
                
                if compile_m.is_present("release") {
                    println!("{}", bfpp::compiler::compile_str_optimized(&infile_content));
                } else {
                    println!("{}", bfpp::compiler::compile_str_unoptimized(&infile_content));
                }
            }
        }
        Some(("interpret", interpret_m)) => {
            if let Some(infile) = interpret_m.value_of("infile") {
                bfpp::interpreter::interpret_str(&read_input_file(infile.to_string()));
            }
        }
        Some(("lex", lex_m)) => {
            if let Some(infile) = lex_m.value_of("infile") {
                let lexems: String = bfpp::compiler::lex_str_to_string(&read_input_file(infile.to_string()));

                if let Some(outfile) = lex_m.value_of("outfile") {
                    write_output_file(outfile.to_string(), lexems);
                } else {
                    println!("{}", lexems);
                }
            }
        }
        Some(("debug", debug_m)) => {
            // implement when debuger is complete
        }
        None => {}
        _ => {}
    }
}