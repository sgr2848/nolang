use super::{interpreter::{MapStruct,ParseTree}};
pub struct ParseTreeNode{
    dat   :  Option<MapStruct>,
    r_child: Option<Box<ParseTreeNode>>,
    l_child :Option<Box<ParseTreeNode>> ,
    parent :  Option<Box<ParseTreeNode>>
}
impl ParseTreeNode{
    pub fn is_leaf(&self)->bool{
        match self.l_child{
            None => match self.r_child{
                None => true,
                _ => false
            }
            _ => false
        }
    }
    pub fn is_null(&self)->bool{
        match self.dat {
            None => true,
            _ => false
        }
    }
}

pub struct DataMapNode{
   
    id : MapStruct,
    parsetree : Box<ParseTree>

}
impl DataMapNode{
    pub fn return_pt(&self)-> Box<ParseTree>{
        return self.parsetree
    }
    
}