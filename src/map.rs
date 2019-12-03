use super::{node::DataMapNode};

pub struct MapVec{
    data: Vec<DataMapNode>
}
impl MapVec{  
    fn get_tree(&self,id_string: String){

        let data_map_node: DataMapNode = Map::get_id().unwrap(); 
        data_map_node.return_pt();
    }
    fn get_data_map_node(&self, id_sting :String)->Option(DataMapNode){

    }
    fn id_exists(self,id_string : String)->bool{
        let iterate : u8 = 0;
        let mut return_bool: bool = false;
        loop{   
                let current_node = self.data[iterate];
                if(current_node.id.get_value == String){
                    return_bool = true;
                    break;     
                }else{
                    iterator += 1;
                }                
        }
        return_bool
    }
}
