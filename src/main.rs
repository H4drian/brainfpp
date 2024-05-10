mod bfpp;

use std::env;
use std::fs;

fn main() {
    let command: Option<String> = env::args().nth(1);

    match command {
        None => panic!("Brainfpp: No command arguments given.");
    }
}
