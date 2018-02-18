/// Represents a chigraph data type
#[derive(Debug, Clone)]
pub enum DataType {
    Int32,
    Float,
    Boolean,
    Struct(String, Vec<NamedDataType>),
}

impl DataType {
    pub fn name(&self) -> &str {
        return match self {
            &DataType::Int32 => "i32",
            &DataType::Float => "float",
            &DataType::Boolean => "bool",
            &DataType::Struct(ref name, _) => &name,
        };
    }
}

/// Represents a data type at runtime
pub trait RuntimeDataType {}

/// Just a data type with a name
#[derive(Debug, Clone)]
pub struct NamedDataType {
    data_type: DataType,
    name: String,
}

impl NamedDataType {
    pub fn new(data_type: DataType, name: &str) -> NamedDataType {
        return NamedDataType {
            data_type,
            name: name.to_string(),
        };
    }

    pub fn name(&self) -> &str {
        return &self.name;
    }
}

/// Trait describing something that can act like a node
/// Can be either pure or impure
pub trait GenericNodeType {
    fn data_inputs(&self) -> Vec<NamedDataType>;
    fn data_outputs(&self) -> Vec<NamedDataType>;

    fn name(&self) -> String;
}

/// A node with exec inputs and exec outputs
pub trait ImpureNodeType: GenericNodeType {
    fn exec_inputs(&self) -> &[&str];
    fn exec_outputs(&self) -> &[&str];
}

/// A node that can be executed
pub trait ExecutableNodeType: GenericNodeType {
    fn execute(&self, data: &[Box<RuntimeDataType>]) -> Vec<Box<RuntimeDataType>>;
}
