use std::collections::VecDeque;
// use regex::Regex;
use std::string::String;
// pub enum Type{
    
//     Digits,
//     Identifiers,
    
// }
// impl Type{
//     fn match_token(&self, string_token: &str){       
        
//     }
// }
pub struct Lexer<'a>{
    pub a_value: &'a String
}
impl<'a> Lexer<'a>{    
    pub fn get_next_token(&self)->VecDeque<String>{
        /*
         *The follwing function takes in string tokenizes them(seperating by whitespace) and returns token in fifo fashion
         */
        let string_to_break =  self.a_value.split_whitespace();
        let mut token_queue:VecDeque<String> = VecDeque::new();
        for token in string_to_break{
            let mut char_vec:Vec<char> = token.chars().collect();
            let rem_V = char_vec.clone();
            let expression_list = ["{","}","+","=","-","/","<",">","(",")"];
            
            for (i,chara) in rem_V.iter().enumerate(){

                let ign = chara.to_string();
                println!("{:}",ign);
                for item in expression_list.iter(){
                    if &ign == item{                        
                        token_queue.push_back(ign.clone());
                        char_vec.remove(i);
                    }
                }
            }
            let token_now : String = char_vec.iter().collect();
            token_queue.push_back(token_now);
        }         
        token_queue
    }
    pub fn token_return(&self, mut token_q : VecDeque<String>)->Option< String>{
        while !token_q.is_empty(){
            let b = token_q.pop_front();
            return b;
        }
        return None;        
    }   
}