use super::{
    // node::{ParseTreeNode,NodeId},
    interpreter::{MapStruct,PTVec}    
};
use std::collections::HashMap;

pub struct MapVec{
    data: HashMap<MapStruct,Option<PTVec>>
}
impl MapVec{  
    pub fn init()->MapVec{
        Self::default()
    }
    pub id_exists(){
        if
    }
}
impl Default for MapVec(){
    fn default()->Self {
        Self{data:HashMap::new()}
    }
}