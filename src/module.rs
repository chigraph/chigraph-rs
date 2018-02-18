use node::DataType;
use node::GenericNodeType;

pub trait Module {
    fn name(&self) -> String;

    fn data_types(&self) -> Vec<&DataType>;
    fn node_types(&self) -> Vec<&GenericNodeType>;
}
