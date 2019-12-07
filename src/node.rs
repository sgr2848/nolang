use super::{
    interpreter::{MapStruct},
    // lexer::Type,
};
#[derive(Clone)]
pub struct NodeId{
    index: u32,
}
impl NodeId{
    
}
#[derive(Clone)]
pub struct ParseTreeNode {
    pub dat: Option<MapStruct>,
    pub r_child: Option<NodeId>,
    pub l_child: Option<NodeId>,
    pub parent: Option<NodeId>,
    pub sibling: Option<NodeId>,
}

impl ParseTreeNode {
    pub fn get_default_node() -> ParseTreeNode {
        ParseTreeNode {
            dat: None,
            r_child: None,
            l_child: None,
            parent: None,
            sibling: None,
        }
    }
    pub fn is_leaf(self) -> bool {
        match self.l_child {
            None => match self.r_child {
                None => true,
                _ => false,
            },
            _ => false,
        }
    }
    pub fn is_null(&self) -> bool {
        match self.dat {
            None => true,
            _ => false,
        }
    }
     
    fn add_data(mut self, datain: MapStruct) -> Self {
        self.dat = Some(datain);
        self
    }
    //These are utilitu function for easier ops
    fn parent(self)->Option<NodeId>{
        self.parent
    }
    fn left_child(self)->Option<NodeId>{
        self.l_child
    }
    fn right_child(self)->Option<NodeId>{
        self.r_child
    }

   
}

pub struct DataMapNode {
    id: MapStruct,
    // parsetree: Box<>,
}
impl DataMapNode {
    // pub fn return_pt(self)  {
    //     return self.parsetree;
    // }
    pub fn return_data(self)->MapStruct{
        return self.id;
    }
}
