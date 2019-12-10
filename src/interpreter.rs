use super::{lexer::Type,node::{NodeId,ParseTreeNode}};
use regex::Regex;
// use std::collections::HashMap;
use std::string::String;

#[derive(Clone)]
pub struct MapStruct {
    value: String,
    typ: Option<Type>,
}
impl MapStruct {
    pub fn get_value(self) -> String {
        return self.value;
    }
    pub fn get_type(self) -> Option<Type> {
        return self.typ;
    }
    pub fn is_equal(&self, new_map: MapStruct) -> bool {
        if self.value == new_map.value {
            return true;
        } else {
            false
        }
    }
    pub fn is_number_or_id(&self) -> bool {
        let check_id_regex = Regex::new(r"(^[a-zA-Z]+[0-9]*)").unwrap();
        let check_digi_regex = Regex::new(r"^\d+$").unwrap();
        if check_digi_regex.is_match(&self.value) || check_id_regex.is_match(&self.value) {
            return false;
        } else {
            true
        }
    }
}
// struct PTVec{
//     nodes: Vec<ParseTreeNode>
// }
// impl PTVec{
//     pub fn new_tree() -> PTVec{
//         Self::default()
//     }
// }
// pub struct Interpret {
//     id_map:HashMap,
// }
// impl Interpret{
//     fn check_id(){

//     }
//     fn parse_sum(&self, int_a: i32, int_b: i32) -> i32 {
//         return int_a + int_b;
//     }
//     fn parse_mut(&self, int_a: i32, int_b: i32) -> i32 {
//         return int_a * int_b;
//     }
//     fn parse_diff(&self, int_a: i32, int_b: i32) -> i32 {
//         return int_a - int_b;
//     }
//     fn parse_div(&self, int_a: i32, int_b: i32) -> i32 {
//         return int_a / int_b;
//     }
//     fn solve_tree()-> Result<T,E>{

//     }
// }
