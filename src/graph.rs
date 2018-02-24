use uuid::Uuid;

use std::collections::HashMap;
use std::borrow::Borrow;

use module::Module;

use node::{NodeType, NamedDataType, DataType};

pub struct NodeInstance {
    id: Uuid,

    pub input_data_connections: Vec<(usize, String)>,
    pub output_exec_connections: Vec<(usize, String)>,

    node_type: Box<NodeType>,
}

impl NodeInstance {
    pub fn new(
        id: Uuid,
        input_data_connections: Vec<(usize, String)>,
        output_exec_connections: Vec<(usize, String)>,
        node_type: Box<NodeType>) -> NodeInstance
    {
        NodeInstance {
            id,
            input_data_connections,
            output_exec_connections,
            node_type,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn data_conn(&self, id: usize) -> Option<&(usize, String)> {
        self.input_data_connections.get(id)
    }

    pub fn exec_conn(&self, id: usize) -> Option<&(usize, String)> {
        self.output_exec_connections.get(id)
    }
}

pub struct GraphFunction {
    name: String,
    nodes: HashMap<Uuid, NodeInstance>,
}

impl GraphFunction {
    pub fn new(name: String, nodes: HashMap<Uuid, NodeInstance>) -> Option<GraphFunction> {
        let ret = GraphFunction {
            name,
            nodes,
        };

        // make sure there is exactly one node entry
        if ret.nodes_with_type("lang:entry").len() != 1 {
            return None;
        }

        // make sure all exit nodes have the same node_type
        {
            let mut exit_node: Option<&NodeType> = None; // node to compare to
            for n in ret.nodes_with_type("lang:exit") {
                if let Some(en) = exit_node {
                    if &n.node_type.data_inputs()[..] != &en.data_inputs()[..] ||
                        &n.node_type.data_outputs()[..] != &en.data_outputs()[..] ||
                        n.node_type.exec_inputs().len() != en.exec_inputs().len() ||
                        n.node_type.exec_outputs().len() != en.exec_outputs().len() {
                        return None;
                    }
                } else {
                    exit_node = Some(n.node_type.borrow());
                }
            }
        }

        Some(ret)
    }

    fn node(&self, id: &Uuid) -> Option<&NodeInstance> {
        self.nodes.get(id)
    }

    fn nodes_with_type(&self, node_type: &str) -> Vec<&NodeInstance> {
        self.nodes.iter().filter(|&(_, n)| n.node_type.name() == node_type)
            .map(|(_, n)| n).collect()
    }

    fn entry_node_type(&self) -> &NodeType {
        self.nodes_with_type("lang:entry")[0].node_type.borrow()
    }

    fn exit_node_type(&self) -> Option<&NodeType> {
        match self.nodes_with_type("lang:exit").get(0) {
            Some(n) => Some(n.node_type.borrow()),
            None => None,
        }
    }
}

impl NodeType for GraphFunction {
    fn name(&self) -> String {
        return self.name.clone();
    }
    fn data_inputs(&self) -> Vec<NamedDataType> {
        self.entry_node_type().data_outputs().clone()
    }
    fn data_outputs(&self) -> Vec<NamedDataType> {
        match self.exit_node_type() {
            Some(nt) => nt.data_inputs().clone(),
            None => vec!(),
        }
    }

    fn exec_inputs(&self) -> Vec<String> {
        self.entry_node_type().exec_outputs().clone()
    }
    fn exec_outputs(&self) -> Vec<String> {
        match self.exit_node_type() {
            Some(nt) => nt.exec_inputs().clone(),
            None => vec!()
        }
    }
}

pub struct GraphModule {
    name: String,
    functions: Vec<GraphFunction>,
}

impl GraphModule {
    pub fn new(name: &str, functions: Vec<GraphFunction>) {
        GraphModule {
            name,
            functions,
        }
    }
}

impl Module for GraphModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn data_types(&self) -> Vec<&DataType> {
        Vec::new()
    }
    fn node_types(&self) -> Vec<&NodeType> {
        self.functions.iter().map(|f| f as &NodeType).collect()
    }
}
