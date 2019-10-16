mod lexer;
use std::string::String;
use std::collections::VecDeque;
// use std::io::{stdin,stdout,Write};
/**
 * here we make the basic 
 */
fn main() {
    let _some_string = String::from("hello = a");
    let _a_token = lexer::Lexer{a_value:&_some_string};
    
    let mut token_vector: VecDeque<String> = _a_token.get_next_token();
    loop{
        println!("{:?}",token_vector);
        let token_now_i = _a_token.token_return(&mut token_vector).unwrap();
        // println!("{:?}",_a_token.token_return(token_vector).unwrap());
        let c = _a_token.return_type(token_now_i);
        println!("{:?}",lexer::Type::match_type(c));   
        if token_vector.is_empty(){
            break;
        }
    }
}
