#[cfg(not(feature = "std"))]
use core::fmt;
#[cfg(feature = "std")]
use std::{fmt,error};
use super::interpreter::{
    MapStruct,
    PTVec,
    
};
#[derive(Clone,Eq,PartialEq,Copy)]
pub struct NodeId {
    pub index: usize,
}
impl NodeId {
    pub fn index(self)->usize{
        self.index
    }
    pub fn rs(some_index:usize)->Self{
        NodeId{index:some_index}
    } 


}
#[derive(Clone,Eq,PartialEq)]
pub struct ParseTreeNode {
    pub data: Option<MapStruct>,
    pub r_child: Option<NodeId>,
    pub l_child: Option<NodeId>,
    pub parent: Option<NodeId>,
    pub sibling: Option<NodeId>,
}

impl ParseTreeNode {
    pub(crate) fn new_node(data_in: MapStruct) -> ParseTreeNode {
        ParseTreeNode {
            data: Some(data_in),
            r_child: None,
            l_child: None,
            parent: None,
            sibling: None,
        }
    }
    pub fn is_full(&self)->bool{
        let bool_ret = self.r_child.is_some() && self.l_child.is_some();
        bool_ret
    }
    pub fn is_leaf(&self) -> bool {
        let bool_ret = self.r_child.is_none() && self.l_child.is_none();
        bool_ret
    }
    pub fn is_null(&self) -> bool {
        match self.data {
            None => true,
            _ => false,
        }
    }
    pub fn get(self)->Option<MapStruct>{
        self.data
    }
    pub fn get_mut(&mut self)->&mut Option<MapStruct>{
        &mut self.data
    }
    //These are utilitu function for easier ops
    pub fn parent(self) -> Option<NodeId> {
        self.parent
    }
    pub fn left_child(self) -> Option<NodeId> {
        self.l_child
    }
    pub fn right_child(self) -> Option<NodeId> {
        self.r_child
    }
}
#[derive(Clone,Copy)]
pub enum CE{
    SiblingE,
    ParentE
} 
impl fmt::Display for CE{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        match self{
            CE::ParentE => f.write_str("MAde a node its parent"),
            CE::SiblingE => f.write_str("Made self its sibling")
        }
    }
} 
#[cfg(feature = "std")]
impl error::Error for CE{}