mod cli;
mod lexer;
mod interpreter;
mod node;
mod mapper;
// mod interpreter; 

use std::collections::{VecDeque,HashMap};
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
    loop {
        print!(">> ");
        let mut input_string = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input_string);
        if let Some('\n') = input_string.chars().next_back() {
            input_string.pop();
            input_string.pop();
            let mut h_map =  mapper::MapVec::init();
            h_map.data.insert(interpreter::MapStruct{
                value: String::from("Hello"),
                typ: Some(lexer::Type::Identifier)
            },interpreter::PTVec{nodes:vec![node::ParseTreeNode{
                data: Some(interpreter::MapStruct{
                    value: String::from("2"),
                    typ: Some(lexer::Type::Digits)
                }),
                r_child:None,
                l_child:None,
                parent:None,
                sibling:None,
            }]});
            let mut new_f = interpreter::Interpret{
                id_map :h_map
            };
            
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
                if !new_f.check_validity(&mut token_vector){
                    println!("***********invalid or unintialized syntax somewhere********")
                }else{
                    loop {
                        // println!("{:?}",token_vector.clone());
                        let mut a = lexer::ConvertInfix{
                            top:-1,
                            some_string:token_vector.clone(),
                            stack:vec!["$".to_string()]
                        }.to_postfix();
                        // let mut token_now_i = _a_token.token_return(&mut token_vector).unwrap();
                        // println!("{:?}",_a_token.token_return(token_vector).unwrap());
                        // let mut c = _a_token.return_type(token_now_i);
                        // println!("{:?}",lexer::Type::match_type(c));
                        println!("{:?}",a.clone());
                        a.pop();
                        token_vector.pop_back();   
                        if a.is_empty() {
                            break;
                        }
                    }
                }
            }
            println!("");
        }
        
    }
}
