extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};

// make this be different for stable and dev branches of the compiler
const VERSION: &str = "indev"

pub fn get_matches() -> ArgMatches {
    App::new("brainfpp")
        .version(VERSION)
        .author("Leon Cotten")
        .about("The official brainfpp compiler/interpreter")
        .subcommand(
            SubCommand::with_name("compile")
                .about("compiles a brainfpp program to brainf")
                .arg(
                    Arg::new("outfile")
                        .short('o')
                        .long("outfile")
                        .value_name("OUTFILE")
                        .takes_value(true)
                        .help("the output file to write to")
                )
                .arg(
                    Arg::new("release")
                        .short('r')
                        .long("release")
                        .takes_value(false)
                        .help("builds the program in release mode (optimized)")
                )
                .arg(
                    Arg::new("dev")
                        .short('r')
                        .long("release")
                        .takes_value(false)
                        .help("builds the program in dev mode (unoptimized)")
                )
        )
        .subcommand(
            SubCommand::with_name("interpret")
                .about("interprets a brainf program")
        )
        .subcommand(
            SubCommand::with_name("lex")
                .about("returns the lexems of a brainfpp program")
                .arg(
                    Arg::new("outfile")
                        .short('o')
                        .long("outfile")
                        .value_name("OUTFILE")
                        .takes_value(true)
                        .help("the output file to write to")
                )
        )
        .subcommand(
            SubCommand::with_name("debug")
                .about("debugging tool for brainfpp. by default it will debug the entire program")
                .arg(
                    Arg::new("start")
                        .short('s')
                        .long("start")
                        .value_name("START")
                        .takes_value(true)
                        .help("the line to start debugging at")
                )
                .arg(
                    Arg::new("end")
                        .short('e')
                        .long("end")
                        .value_name("END")
                        .takes_value(true)
                        .help("the line to end debugging at")
                )
        )
    .get_matches()
}