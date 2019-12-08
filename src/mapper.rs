use super::{   
    interpreter::{MapStruct,PTVec}    
};
use std::collections::HashMap;
#[derive(Clone,Eq,PartialEq)]
pub struct MapVec{
    data: HashMap<MapStruct,Option<PTVec>>
}
impl MapVec{  
    pub fn init()->MapVec{
        Self::default()
    }
}
impl Default for MapVec{
    fn default()->Self {
        Self{data:HashMap::new()}
      
    }
}