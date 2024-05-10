mod bfpp;

use std::env;
use std::fs;

fn main() {
    let file_path: String = env::args().collect::<Vec<String>>().get(1).unwrap().to_string();
    let file_content: String = fs::read_to_string(file_path).unwrap();

}
