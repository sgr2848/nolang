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
    pub fn value_exists(self,m_val: MapStruct)->bool{
        if self.data.contains_key(&m_val){
            true
        }else{
            false
        }
    }
}
impl Default for MapVec{
    fn default()->Self {
        Self{data:HashMap::new()}      
    }
}