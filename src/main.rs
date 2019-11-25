mod cli;
mod lexer;

use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};
use std::string::String;
fn main() {
    match cli::start_nano() {
        Ok(a) => println!("{}", a),
        Err(e) => println!("{}", e),
    }
    loop {
        print!(">> ");
        let mut input_string = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input_string);
        if let Some('\n') = input_string.chars().next_back() {
            input_string.pop();
            input_string.pop();
            if input_string == "exit" || input_string == ":q" {
                break;
            }
            else if input_string.is_empty(){
                print!("");
            } 
            else {
                let _a_token = lexer::Lexer {
                    a_value: &input_string,
                };
                let mut token_vector: VecDeque<String> = _a_token.get_token_queue();
                loop {
                    let t_n_i = _a_token.token_return(&mut token_vector).unwrap();
                    let r = t_n_i.clone();
                    let t_r = _a_token.return_type(t_n_i);
                    print!("{:?} -->", r);
                    println!("{:?}", lexer::Type::match_type(t_r));
                    if token_vector.is_empty() {
                        break;
                    }
                }
            }
            println!("");
        }
        
    }
}
