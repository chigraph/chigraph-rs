use node::DataType;
use node::NodeType;

pub trait Module {
    fn name(&self) -> String;

    fn data_types(&self) -> Vec<&DataType>;
    fn node_types(&self) -> Vec<&NodeType>;
    
    fn node_type(&self, name: &str) -> Option<&NodeType> {
        for d in self.data_types {
            if d.name == name {
                return Some(&d);
            }
        }
        None
    }
}
