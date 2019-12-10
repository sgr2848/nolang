mod cli;
mod interpreter;
mod lexer;
mod mapper;
mod node;
// mod interpreter;

use std::collections::{HashMap, VecDeque};
use std::io::{stdin, stdout, Write};
use std::string::String;
fn main() {
    /*TODO Main
            []make hmap for id and mapstruct
            []]make interpreter id validation
    */
    match cli::start_nano() {
        Ok(a) => println!("{}", a),
        Err(e) => println!("{}", e),
    }

    let mut new_f = interpreter::Interpret {
        id_map: mapper::MapVec::init(),
    };
    &mut new_f.id_map.insert_val(String::from("Hello"),vec!["2".to_string()]);
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
            } else if input_string.is_empty() {
                print!("");
            } else {
                let _a_token = lexer::Lexer {
                    a_value: &input_string,
                };
                let mut token_vector: VecDeque<String> = _a_token.get_token_queue();
                let rar =&mut new_f.check_validity(&mut token_vector) ;
                for (keys,value) in new_f.clone().id_map.data{
                        println!(" the keys-->{:?}",keys.get_value())
                    }
                if *rar {
                    loop {
                        // println!("{:?}",token_vector.clone());
                        let mut a = lexer::ConvertInfix {
                            top: -1,
                            some_string: token_vector.clone(),
                            stack: vec!["$".to_string()],
                        }
                        .to_postfix();
                        a.pop();
                        token_vector.pop_back();
                        if a.is_empty() {
                            break;
                        }
                    }
                    
                    
                } else {
                    println!("***********invalid or unintialized syntax somewhere********");
                }
            }
            println!("");
        }
    }
}
