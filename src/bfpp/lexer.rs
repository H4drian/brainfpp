#[derive(Debug)]
enum Token {
    Sdp,        // set data pointer
    Add,        // add data
    Sub,        // sub from data
    Out,        // output 
    Inn,        // input
    Slp,        // start loop
    Elp,        // end loop
    Noop        // no op
}

#[derive(Debug)]
pub struct Lexem {
    token: Token,
    arg: Option<usize>,
    line: usize
}

pub fn lex_str(source_code: &str) -> Vec<Lexem> {
    let mut lexem_vec: Vec<Lexem> = Vec::new();

    let lines: Vec<&str> = source_code.lines().collect();
    let mut line_count: usize = 0;

    for line in lines {
        line_count += 1;

        let instruction: Token = match line.split_whitespace().next().unwrap_or("").to_lowercase().as_str() {
            "sdp" => Token::Sdp,
            "add" => Token::Add,
            "sub" => Token::Sub,
            "out" => Token::Out,
            "inn" => Token::Inn,
            "["   => Token::Slp,
            "]"   => Token::Elp,
            _     => Token::Noop, 
        };
        let instruction_arg: Option<usize> = line

            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok());

        lexem_vec.push(Lexem {
            token: instruction,
            arg: instruction_arg,
            line: line_count
        });
    }

    lexem_vec
}