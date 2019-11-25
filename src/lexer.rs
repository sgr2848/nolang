use regex::Regex;
use std::collections::VecDeque;
use std::string::String;
#[derive(Debug)]
pub enum Type {
    /*
     *The type of token to return to interpreter
     */
    Digits,
    Identifier,
    Plus,
    Minus,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    Equals,
    // EndOfLine,
    SemiColon,
    NewLine,
    Whoknows,
}

impl Type {
    //this might be a little redundant, but ehh
    pub fn match_type(a_type: Option<Type>) -> String {
        match a_type {
            Some(Type::Plus) => String::from("Plus"),
            Some(Type::Minus) => String::from("Minus"),
            Some(Type::LeftParenthesis) => String::from("Parenthesis"),
            Some(Type::RightParenthesis) => String::from("Parenthesis"),
            Some(Type::Divide) => String::from("Divide"),
            Some(Type::Digits) => String::from("Digits"),
            Some(Type::Whoknows) => String::from("who_knows"),
            Some(Type::SemiColon) => String::from("Semi colon"),
            Some(Type::NewLine) => String::from("newline"),
            Some(Type::Equals) => String::from("Equals"),
            Some(Type::Identifier) => String::from("Identifier"),
            // Some(Type::EndOfLine) => String::from("EOL"),
            None => String::from("Nan"),
        }
    }
}

pub struct Lexer<'a> {
    pub a_value: &'a String,
}

impl<'a> Lexer<'a> {
    pub fn get_token_queue(&self) -> VecDeque<String> {
        /*
         *The follwing function takes in string tokenizes them(seperating by whitespace) and returns token in fifo fashion
         */
        let string_to_break = self.a_value.split_whitespace();
        let mut token_queue: VecDeque<String> = VecDeque::new();
        for token in string_to_break {
            let mut char_vec: Vec<char> = token.chars().collect();
            let rem_v = char_vec.clone();
            let expression_list = ["{", "}", "+", "=", "-", "/", "<", ">", "(", ")", ";"];
            for (i, chara) in rem_v.iter().enumerate() {                
                let ign = chara.to_string();
                // dbg!(&ign);
                for item in expression_list.iter() {
                    if &ign == item {
                        token_queue.push_back(ign.clone());
                        char_vec.remove(i);    
                        }                    
                    }
                }            
      
            let token_now: String = char_vec.iter().collect();
            token_queue.push_back(token_now);
        }
        token_queue
    }
    pub fn matches_digit_and_id(a_string: &str) -> Option<Type> {
        /*
        Function that matches the string to either digits or identifier
        ____________________
        Parameter: 
            a_string : &str
        ____________________
        Return:
            Some(Type) | None
        */ 
        let digit_regex = Regex::new(r"^\d+$").unwrap();
        let identifier_regex = Regex::new(r"^[a-zA-Z]+[0-9]*").unwrap();
        // dbg!(a_string);
        if digit_regex.is_match(a_string) {
            return Some(Type::Digits);
        } else if identifier_regex.is_match(a_string) {
            return Some(Type::Identifier);
        } else {
            return None;
        }
    }
    pub fn return_type(&self, some_string: String) -> Option<Type> {
        /*
        Function return Type of the given string
        ____________________
        Parameter: 
            some_string : String
        ____________________
        Return:
            Type
        */        
        match some_string.as_str() {
            "+" => Some(Type::Plus),
            "-" => Some(Type::Minus),
            "(" => Some(Type::LeftParenthesis),
            ")" => Some(Type::RightParenthesis),
            "/" => Some(Type::Divide),
            ";" => Some(Type::SemiColon),
            "=" => Some(Type::Equals),
            "\n" => Some(Type::NewLine),
            _ => Lexer::matches_digit_and_id(some_string.as_str()),
        }
    }

    pub fn token_return(&self, token_q: &mut VecDeque<String>) -> Option<String> {
        /*
        Function returns a string to use in the interpreter untill the queue is empty, return None if empty
                ____________________
        Parameter: 
            token_q : VecDeque(must be mutable)
        ____________________
        Return:
            Type: Some(String)|None
        */

        if !token_q.is_empty() {
            let b = token_q.pop_front();
            return b;
        } else {
            return None;
        }
    }
}
