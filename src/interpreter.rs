#![allow(dead_code, unused_imports)]
use super::{
    lexer::{ConvertInfix, Lexer, Type},
    mapper::MapVec,
    node::{NodeId, ParseTreeNode, CE},
};
use regex::Regex;
use std::collections::HashMap;
use std::{
    collections::VecDeque,
    error, fmt, mem,
    ops::{Index, IndexMut},
    string::String,
};

#[derive(Clone, Eq, PartialEq, Hash)]

pub struct MapStruct {
    pub value: String,
    pub typ: Option<Type>,
}
impl MapStruct {
    pub fn new_struct(some_string: String) -> MapStruct {
        MapStruct {
            value: some_string.clone(),
            typ: Lexer::return_type(some_string.clone()),
        }
    }
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
#[derive(Clone, Eq, PartialEq)]
pub struct PTVec {
    pub nodes: Vec<ParseTreeNode>,
}
impl PTVec {
    pub fn new_tree() -> PTVec {
        Self::default()
    }
    pub fn get(&self, id: NodeId) -> Option<&ParseTreeNode> {
        self.nodes.get(id.index())
    }
    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut ParseTreeNode> {
        self.nodes.get_mut(id.index())
    }
    pub fn new_node(&mut self, data: MapStruct) -> NodeId {
        let next_id = self.nodes.len();
        self.nodes.push(ParseTreeNode::new_node(data));
        NodeId::rs(next_id)
    }
    pub fn iter(&self) -> impl Iterator<Item = &ParseTreeNode> {
        self.nodes.iter()
    }
}
impl Default for PTVec {
    fn default() -> Self {
        Self { nodes: Vec::new() }
    }
}
impl Index<NodeId> for PTVec {
    type Output = ParseTreeNode;
    fn index(&self, node: NodeId) -> &ParseTreeNode {
        &self.nodes[node.index()]
    }
}
impl IndexMut<NodeId> for PTVec {
    fn index_mut(&mut self, node: NodeId) -> &mut ParseTreeNode {
        &mut self.nodes[node.index()]
    }
}
pub(crate) fn insert_node_efi(pt_vec:&mut PTVec,new_id:NodeId,operand_stack:&mut Vec<NodeId>){
    
    let current_n = pt_vec.get(new_id).unwrap();
    if Type::match_id_digits(current_n.clone().data.unwrap().get_type()){
       operand_stack.push(new_id); 
    }else{
        let right = operand_stack.pop();
        let left = operand_stack.pop();
        let mut parent_ptn = pt_vec.get_mut(new_id).unwrap();
        parent_ptn.r_child = right;
        parent_ptn.l_child = left;
        
        let mut right_ptn = pt_vec.get_mut(right.unwrap()).unwrap();
        right_ptn.sibling = left;
        right_ptn.parent = Some(new_id);
        
        let mut left_ptn = pt_vec.get_mut(left.unwrap()).unwrap();
        left_ptn.parent = Some(new_id);
        left_ptn.sibling = right;
        operand_stack.push(new_id);
    }
}

pub(crate) fn build_pt(mut pt_vec: PTVec, mut stack_v: VecDeque<String>) -> PTVec {
    // let mut root_string: String = stack_v.pop().unwrap();
    // let _root_a_t_m = pt_vec.new_node(MapStruct::new_struct(root_string.clone()));
    // dbg!(_root_a_t_m.clone().index());
    let mut operand_stack = Vec::new();
    for i in stack_v.clone().iter() {
        dbg!(i);        
        let mut current_string: String = stack_v.pop_front().unwrap();
        let mut _node_a_t_m = pt_vec.new_node(MapStruct::new_struct(current_string.clone()));
        insert_node_efi(&mut pt_vec, _node_a_t_m, &mut operand_stack)
    }
    pt_vec
}
// pub(crate) fn prune_tree(&mut pt_vec: PTVec,n: NodeId)->PTVec{
// }

#[derive(Clone, Eq, PartialEq)]
pub struct Interpret {
    pub id_map: MapVec,
}
impl Interpret {
    pub fn check_validity(&mut self, _stream_vec: &mut VecDeque<String>) -> bool {
        let mut b_val = true;
        let index: usize = 0;
        let mut _current_token = &_stream_vec[index];
        if _stream_vec.len() == 1 as usize
            && Type::match_id(Lexer::return_type(_current_token.clone()))
        {
            if self.id_map.clone().value_exists(MapStruct {
                value: _current_token.clone(),
                typ: Lexer::return_type(_current_token.clone()),
            }) {
                return true;
            } else {
                return false;
            }
        } else if _stream_vec.len() >= 1
            && Type::match_id(Lexer::return_type(_current_token.clone()))
            && !self.id_map.clone().value_exists(MapStruct {
                value: _current_token.clone(),
                typ: Lexer::return_type(_current_token.clone()),
            })
        {
            if Type::is_eq_type(Lexer::return_type(_stream_vec[index + 1].clone())) {
                let id_name = _stream_vec.pop_front();
                for _ in 0..1 {
                    _stream_vec.pop_front();
                }
                let mut postfix_vec = ConvertInfix {
                    top: -1,
                    some_string: _stream_vec.clone(),
                    stack: vec!["$".to_string()],
                }
                .to_postfix();
                dbg!(postfix_vec.clone());

                &mut self
                    .id_map
                    .insert_val(id_name.clone().unwrap(), postfix_vec);
                return true;
            } else {
                return false;
            }
        } else {
            for i in 0..(_stream_vec.len() - 1) {
                if Type::match_digits(Lexer::return_type(_stream_vec[i].clone())) {
                    b_val = true;
                } else if !Type::is_eq_type(Lexer::return_type(_stream_vec[i].clone()))
                    && self.id_map.clone().value_exists(MapStruct {
                        value: _current_token.clone(),
                        typ: Lexer::return_type(_current_token.clone()),
                    })
                {
                    b_val = true;
                } else {
                    b_val = false;
                    break;
                }
            }
            b_val
        }
    }
    fn parse_sum(int_a: i32, int_b: i32) -> i32 {
        return int_a + int_b;
    }
    fn parse_mut(int_a: i32, int_b: i32) -> i32 {
        return int_a * int_b;
    }
    fn parse_diff(int_a: i32, int_b: i32) -> i32 {
        return int_a - int_b;
    }
    fn parse_div(int_a: i32, int_b: i32) -> i32 {
        return int_a / int_b;
    }
}
