mod lexer;
use std::string::String;
use std::collections::VecDeque;
fn main() {
    let _some_string = String::from("2a");
    let _a_token = lexer::Lexer{a_value:&_some_string};
    
    let token_vector: VecDeque<String> = _a_token.get_next_token();  
    println!("{:?}",token_vector);
    let token_now_i = _a_token.token_return(token_vector).unwrap();
    // println!("{:?}",_a_token.token_return(token_vector).unwrap());
    let c = _a_token.return_type(token_now_i);

    println!("{:?}",lexer::Type::match_type(c));

}
