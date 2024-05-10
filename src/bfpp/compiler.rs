use crate::bfpp::lexer;
use lexer::Token;
use lexer::Lexem;

pub fn compile_str(source_code: &str) -> String {
    let mut output_string: String = String::new();
    let lexems: Vec<Lexem> = lexer::lex_str(source_code);
    let mut data_ptr: usize = 0;

    for lexem in lexems {
        let instruction: Token = lexem.token;
        let arg: Option<usize> = lexem.arg;
        let line: usize = lexem.line;

        match instruction {
            Token::Sdp => {
                if arg == None {
                    panic!("Brainfpp: Error at line {line}: No data address given");
                }

                let offset: isize = (arg.unwrap() as isize) - (data_ptr as isize);

                if offset > 0 {
                    output_string.push_str(&">".repeat(offset as usize));
                    data_ptr += offset as usize;
                } else if offset < 0 {
                    output_string.push_str(&"<".repeat(offset as usize));
                    data_ptr -= offset as usize;
                } else {}
            }
            Token::Add => output_string.push_str(&"+".repeat(arg.unwrap())),
            Token::Sub => output_string.push_str(&"-".repeat(arg.unwrap())),
            Token::Out => output_string.push_str(&"."),
            Token::Inn => output_string.push_str(&","),
            Token::Slp => output_string.push_str(&"["),
            Token::Elp => output_string.push_str(&"]"),
            Token::Noop => {}
        }
    }

    output_string
}