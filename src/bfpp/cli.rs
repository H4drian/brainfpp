/*
 * MIT License
 * 
 * Copyright (c) 2024 Leon Cotten
 * 
 * This language is provided under the MIT Licence.
 * See LICENSE for more information.
 */ 

extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

const VERSION: &str = "indev";

pub fn get_matches() -> ArgMatches {
    // arguments
    let infile_arg: Arg = Arg::with_name("infile")
        .value_name("INFILE")
        .help("input file to compile")
        .required(true)
        .index(1);

    let outfile_arg: Arg = Arg::new("outfile")
        .short('o')
        .long("outfile")
        .value_name("OUTFILE")
        .takes_value(true)
        .help("the output file to write to");

    let link_arg: Arg = Arg::new("link")
        .short('l')
        .long("link")
        .takes_value(true)
        .value_name("lib")
        .help("links a brainfpp file to the compiler")
        .multiple(true)
        .required(false);

    let no_std_arg: Arg = Arg::new("no-std")
        .long("no-std")
        .takes_value(false)
        .help("dissable default linking of brainfpp standard library")
        .required(false);

    // CLI app
    App::new("brainfpp")
        .version(VERSION)
        .author("Leon Cotten")
        .about("The official brainfpp compiler/interpreter")
        .subcommand(
            SubCommand::with_name("compile")
                .about("compiles a brainfpp program to brainf")
                .arg(infile_arg.clone())
                .arg(outfile_arg.clone())
                .arg(link_arg.clone())
                .arg(no_std_arg.clone())
                .arg(
                    Arg::new("release")
                        .short('r')
                        .long("release")
                        .takes_value(false)
                        .help("builds the program in release mode (optimized)")
                )
                .arg(
                    Arg::new("dev")
                        .short('d')
                        .long("dev")
                        .takes_value(false)
                        .help("builds the program in dev mode (unoptimized)")
                )
        )
        .subcommand(
            SubCommand::with_name("interpret")
                .about("interprets a brainf program")
                .arg(infile_arg.clone())
        )
        .subcommand(
            SubCommand::with_name("lex")
                .about("returns the lexems of a brainfpp program")
                .arg(infile_arg.clone())
                .arg(outfile_arg.clone())
                .arg(link_arg.clone())
                .arg(no_std_arg.clone())
        )
    .get_matches()
}