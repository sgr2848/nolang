use std::collections::VecDeque;

pub struct Lexer<'a>{
    pub a_value: &'a str
}
impl<'a> Lexer<'a>{    
    pub fn get_next_token(&self)->VecDeque<&str>{
        /*
         *The follwing function takes in string tokenizes them(seperating by whitespace) and returns token in fifo fashion
         */
        let string_to_break =  self.a_value.split_whitespace();
        let mut token_queue:VecDeque<&str> = VecDeque::new();
        for token in string_to_break{
            token_queue.push_back(token);
        }         
        token_queue
    }
    pub fn token_return(&self, mut token_q : VecDeque<&'a str>)->Option<&'a str>{
        while !token_q.is_empty(){
            let b = token_q.pop_front();
            return b;
        }
        return None;        
    }   
}