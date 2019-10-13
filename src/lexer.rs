use std::collections::VecDeque;
use regex::Regex;
use std::string::String;
pub enum Type{    
/*
 *The type of token to return to interpreter 
 */
    Digits,
    Identifiers,
    Plus,
    Minus,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    whoknows
}
impl Type{
    //this might be a little redundant, but ehh
    pub fn match_type(a_type:Type)->String{
        match a_type{
            Type::Plus=> String::from("Plus"),
            Type::Minus=> String::from("Minus"),           
            Type::LeftParenthesis=> String::from("Parenthesis"),
            Type::RightParenthesis=> String::from("Parenthesis"),
            Type::Divide => String::from("Divide"),
            _ => String::from("who_knows")
        }
    }
}

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
    pub fn return_type(&self,some_string:String)->Option<Type>{
        let digit_regex = Regex::new(r"\d+").unwrap();
        // let is_identifier =Regex::new(r"");
        match some_string.as_str(){
            "+" => Some(Type::Plus),
            "-"=> Some(Type::Minus),            
            "("=> Some(Type::LeftParenthesis),
            ")"=> Some(Type::RightParenthesis),
            "/" => Some(Type::Divide),
            _ => Some(Type::whoknows)
        };
        if digit_regex.is_match(some_string.as_str()){
            return Some(Type::Digits);
        }
        else{
            return None
        }

        
    }   
    pub fn token_return(&self, mut token_q : VecDeque<String>)->Option<String>{
        if !token_q.is_empty(){
            let b = token_q.pop_front();
            return b;
        }
        else{
            return None;

        } 
    }

}