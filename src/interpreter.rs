
use super::{lexer::Type,node::ParseTreeNode};
use std::string::String;
use std::collections::{HashMap};
static PRECEDENCE_MAP: HashMap<String,i8> = [ ("+",1),("-",1),("*",2),("/",2)].iter().cloned().collect();

pub struct MapStruct{
    value : String , 
    typ: Option<Type>
}
impl MapStruct{
    pub fn get_value(&self)->String{
        return self.value
    }
    pub fn get_type(&self)->Option<Type>{
        return self.typ
    }
    pub fn is_equal(&self,new_map:MapStruct)->bool{
        if self.value == new_map.value {
            return true;
        }else{
            false
        }
    }
}
pub struct ParseTree{
   data:Option<ParseTreeNode>
}
impl ParseTree{
    pub fn create_new(&self,val:String,tp:Type)->ParseTree{
        match self.data{
            None => ParseTree{data :Some(ParseTreeNode{
                dat: Some(MapStruct{value:val, typ : Some(tp)}),
                r_child: None,
                l_child: None,
                parent : None,
            })},
            _ => ParseTree::add_child(self, MapStruct{value:val, typ : Some(tp)}),
        }
    }
    // pub fn add_child(&self, mapVal: MapStruct)->ParseTree{
    //     // match mapVal.t{
    //     //     Some(Type::Plus)

    //     // }
    // }
    fn parse_sum(&self,int_a: i32, int_b: i32) -> i32{
        return int_a + int_b
    }
    fn parse_mut(&self,int_a: i32, int_b: i32) -> i32{
        return int_a * int_b
    }
    fn parse_diff(&self,int_a: i32, int_b: i32) -> i32{
        return int_a - int_b
    }
    fn parse_div(&self,int_a: i32, int_b: i32) -> i32{
        return int_a / int_b
    }      
}
