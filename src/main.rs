mod lexer;
use std::string::String;
use std::collections::VecDeque;
fn main() {
    let _some_string : String = "a+b".to_string();
    let _a_token = lexer::Lexer{a_value:&_some_string};
    let token_vector: VecDeque<String> = _a_token.get_next_token();  
    println!("{:?}",token_vector);
    println!("{:?}",_a_token.token_return(token_vector).unwrap());

}
