use super::{
    lexer::{ConvertInfix, Lexer, Type},
    mapper::MapVec,
    node::{NodeId, ParseTreeNode, CE},
};
use regex::Regex;

use std::collections::HashMap;
use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
    string::String,
};

#[derive(Clone, Eq, PartialEq, Hash)]

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
pub(crate) fn insert_node(
    mut pt_vec: PTVec,
    new_id: NodeId,
    parent: Option<NodeId>,
    sibling: Option<NodeId>,
) -> Result<(), CE> {
    if sibling == Some(new_id) {
        return Err(CE::SiblingE);
    }
    if parent == Some(new_id) {
        return Err(CE::ParentE);
    }
    if new_id.index() != 0 as usize {
        let parent_node = pt_vec.get_mut(parent.unwrap()).unwrap();
        let parent_map_struct_type = parent_node.clone().get().unwrap().get_type();
        if parent_node.clone().is_leaf() && !Type::match_id_digits(parent_map_struct_type) {
            parent_node.l_child = Some(new_id);
            let mut current_node = &mut pt_vec.get_mut(new_id).unwrap();
            current_node.parent = parent;
        } else if !parent_node.clone().is_leaf() && !Type::match_id_digits(parent_map_struct_type) {
            if parent_node.clone().left_child().is_none()
                && parent_node.clone().right_child().is_some()
            {
                parent_node.l_child = Some(new_id);
                let mut current_node = &mut pt_vec.get_mut(new_id).unwrap();
                current_node.parent = parent;
            } else if parent_node.clone().right_child().is_none()
                && parent_node.clone().left_child().is_some()
            {
                parent_node.r_child = Some(new_id);
                let mut current_node = &mut pt_vec.get_mut(new_id).unwrap();
                current_node.parent = parent;
            }
        }
    }
    Ok(())
}
#[derive(Clone, Eq, PartialEq)]
pub struct Interpret {
    pub id_map: MapVec,
}
impl Interpret {
    // fn insert_value()
    pub fn check_validity(self, mut _stream_vec: VecDeque<String>) -> bool {
        let mut b_val = true;
        let index: usize = 0;
        let mut _current_token = &_stream_vec[index];
        if _stream_vec.len() == 1 as usize
            && Type::match_id(Lexer::return_type(_current_token.clone()))
        {
            if self.id_map.value_exists(MapStruct {
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
    fn parse_sum(&self, int_a: i32, int_b: i32) -> i32 {
        return int_a + int_b;
    }
    fn parse_mut(&self, int_a: i32, int_b: i32) -> i32 {
        return int_a * int_b;
    }
    fn parse_diff(&self, int_a: i32, int_b: i32) -> i32 {
        return int_a - int_b;
    }
    fn parse_div(&self, int_a: i32, int_b: i32) -> i32 {
        return int_a / int_b;
    }
}
