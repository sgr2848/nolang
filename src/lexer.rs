use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::string::String;
#[derive(Debug, Clone, Copy)]

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
    Multiply,
    // EndOfLine,
    SemiColon,
    NewLine,
    Whoknows,
}
impl Type {
    //this might be a little redundant, but ehh
    pub fn match_type(a_type: Option<Type>) -> String {
        match a_type {
            Some(Type::Multiply) => String::from("Multiply"),
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
    pub fn match_id_digits(some_type: Option<Type>) -> bool {
        match some_type {
            Some(Type::Identifier) => true,
            Some(Type::Digits) => true,
            _ => false,
        }
    }
    pub fn is_eq_type(some_type: Option<Type>) -> bool {
        match some_type {
            Some(Type::Equals) => true,
            _ => false,
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
            let mut i_string = String::new();
            let mut char_vec: Vec<char> = token.chars().collect();
            let rem_v = char_vec.clone();
            let expression_list = ["{", "}", "+", "=", "-", "/", "<", ">", "(", ")", "*"];
            let reg_id = Regex::new(r"[a-zA-Z]").unwrap();
            let digi_id = Regex::new(r"\d").unwrap();
            for charcter in rem_v.iter(){
                if reg_id.is_match(&charcter.to_string()) || digi_id.is_match(&charcter.to_string()){
                    i_string.push(*charcter);
                }else{
                    dbg!(i_string.clone());
                    token_queue.push_back(i_string.clone());
                    while !i_string.is_empty(){
                        i_string.remove(i_string.len() -1);                        
                    }
                    token_queue.push_back(charcter.to_string());
                }               
            }
            token_queue.push_back(i_string);

            // for (i, chara) in rem_v.iter().enumerate() {
            //     let ign = chara.to_string();
            //     // dbg!(&ign);
            //     for item in expression_list.iter() {
            //         if &ign == item {
            //             token_queue.push_back(ign.clone());
            //             char_vec.remove(i);
            //         }
            //     }
            // }
            // let token_now: String = char_vec.iter().collect();
            // token_queue.push_back(token_now);
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
            "*" => Some(Type::Multiply),
            "+" => Some(Type::Plus),
            "-" => Some(Type::Minus),
            "(" => Some(Type::LeftParenthesis),
            ")" => Some(Type::RightParenthesis),
            "/" => Some(Type::Divide),
            ";" => Some(Type::SemiColon),
            "=" => Some(Type::Equals),
            "\\n" => Some(Type::NewLine),
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
pub struct ConvertInfix {
    pub top: i8,
    pub some_string: VecDeque<String>,
    pub stack: Vec<String>,
}
impl ConvertInfix {
    fn is_empty(&self) -> bool {
        if self.top == -1 {
            return true;
        } else {
            false
        }
    }
    fn peek(&self) -> String {
        let r_val = self.stack[self.stack.len() - 1].clone();
        r_val
    }

    fn pop(&mut self) -> String {
        if self.is_empty() {
            return "$".to_string();
        } else {
            self.top -= 1;
            self.stack.pop().unwrap()
        }
    }
    fn push(&mut self, string_to_add: String) {
        self.top += 1;
        self.stack.push(string_to_add);
    }
    fn is_operand(&self, some_chr: String) -> bool {
        let a_c_vec: Vec<char> = some_chr.chars().collect();
        let mut boolval: bool = false;
        for a_char in a_c_vec {
            boolval = a_char.is_alphabetic() ;
        }
        boolval
    }
    fn not_greater(&self, some_chr: String) -> bool {
        let mut hash_map = HashMap::new();
        hash_map.insert(String::from("+"), 1);
        hash_map.insert(String::from("-"), 1);
        hash_map.insert(String::from("*"), 2);
        hash_map.insert(String::from("/"), 2);
        if hash_map.contains_key(&some_chr) && hash_map.contains_key(&self.peek()) {
            let a: i8 = hash_map[&some_chr];
            let b: i8 = hash_map[&self.peek()];
            if a <= b {
                return true;
            } else {
                return false;
            }
        } else {
            false
        }
    }
    pub fn to_postfix(&mut self) -> Vec<String> {
        let mut return_vector = Vec::new();
        for i in self.some_string.clone().iter() {
            if self.is_operand(i.to_string()) {
                return_vector.push(i.to_string());
            } else if i.to_string() == "(".to_string() {
                self.push(i.to_string())
            } else if i.to_string() == ")".to_string() {
                while (!self.is_empty()) && (self.peek() != "(") {
                    let a: String = self.pop();
                    return_vector.push(a);
                }
                if !self.is_empty() && self.peek() != "(" {
                    println!("Something");
                } else {
                    self.pop();
                }
            } else {
                while (!self.is_empty()) && self.not_greater(i.to_string()) {
                    return_vector.push(self.pop());
                }
                self.push(i.to_string());
            }
        }
        while !self.is_empty() {
            return_vector.push(self.pop());
        }
        return_vector
    }
}
