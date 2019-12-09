use super::{   
    interpreter::{
        MapStruct,
        PTVec,
        // create_pt
    },
    lexer::{
        Type,
        Lexer},
    node::{
        NodeId,
        ParseTreeNode,

    }    
};
#[cfg(not(feature = "std"))]
use core::fmt;
#[cfg(feature = "std")]
use std::{fmt,error};
use std::collections::{
    HashMap,
    VecDeque};
#[derive(Clone,Eq,PartialEq)]
pub struct MapVec{
    pub data: HashMap<MapStruct,PTVec>
}
impl MapVec{  
    pub fn init()->MapVec{
        Self::default()
    }
    pub fn value_exists(self,m_val: MapStruct)->bool{
        if self.data.contains_key(&m_val){
            true
        }else{
            false
        }
    }
    pub fn insert_val(mut self,some_id : String,mut some_vec:Vec<String>)->Result<(),MapVecError>{
        if Type::match_id(Lexer::return_type(some_id.clone())){
            return Err(MapVecError::NotID);
        }
        let id_map_struct = MapStruct{
            value: some_id.clone(),
            typ: Lexer::return_type(some_id)
        };
        let mut parse_tree_vec  = PTVec::new_tree();
        let new_ptvec = interpreter::build_pt(parse_tree_vec, some_vec);
        self.data.insert(id_map_struct, new_ptvec.clone());
        Ok(())
        
    }
}
impl Default for MapVec{
    fn default()->Self {
        Self{data:HashMap::new()}      
    }
}
#[derive(Clone,Copy)]
pub enum MapVecError{
    NotID
}
impl fmt::Display for MapVecError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        match self{
            MapVecError::NotID =>f.write_str("Not and ID")
        }
    }
}
#[cfg(feature = "std")]
impl error::Error for MapVecError{
    
}