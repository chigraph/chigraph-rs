// Data types

use node::GenericNodeType;
use node::DataType;
use node::NamedDataType;

use module::Module;

use std::borrow::Borrow;

#[derive(Clone)]
enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinOp {
    fn string_representation(&self) -> &'static str {
        return match self {
            &BinOp::Add => "+",
            &BinOp::Subtract => "-",
            &BinOp::Multiply => "*",
            &BinOp::Divide => "/",
        };
    }
}

struct BinOpNodeType {
    data_type: DataType,
    op: BinOp,
}

impl GenericNodeType for BinOpNodeType {
    fn data_inputs(&self) -> Vec<NamedDataType> {
        return vec!(
            NamedDataType::new(self.data_type.clone(), "a"),
            NamedDataType::new(self.data_type.clone(), "b"),
        );
    }
    fn data_outputs(&self) -> Vec<NamedDataType> {
        return vec!(
            NamedDataType::new(self.data_type.clone(), ""),
        );
    }

    fn name(&self) -> String {
        return format!("{0}{1}{0}", self.data_type.name(), self.op.string_representation());
    }
}

pub struct LangModule {
    node_types: Vec<Box<GenericNodeType>>,
    data_types: Vec<DataType>,
}

impl LangModule {
    pub fn new() -> LangModule {
        let ops = vec!(BinOp::Add, BinOp::Subtract, BinOp::Multiply, BinOp::Divide);

        let mut ret = LangModule {
            node_types: Vec::<Box<GenericNodeType>>::new(),
            data_types: vec!(
                DataType::Int32,
                DataType::Float,
                DataType::Boolean,
            ),
        };

        for data_type in &ret.data_types {
            for op in &ops {
                ret.node_types.push(Box::new(BinOpNodeType {
                    data_type: data_type.clone(),
                    op: op.clone(),
                }))
            }
        }

        ret
    }
}

impl Module for LangModule {
    fn name(&self) -> String {
        return "lang".to_string();
    }

    fn data_types(&self) -> Vec<&DataType> {
        let mut ret = Vec::<&DataType>::new();

        for dt in &self.data_types {
            ret.push(dt.borrow());
        }

        return ret;
    }
    fn node_types(&self) -> Vec<&GenericNodeType> {
        let mut ret = Vec::<&GenericNodeType>::new();

        for nt in &self.node_types {
            ret.push(nt.borrow());
        }

        return ret;
    }
}
