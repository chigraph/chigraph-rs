use uuid::Uuid;

use std::collections::HashMap;

use module::Module;

use node::NodeType;

struct NodeInstance {
    id: Uuid,

    input_data_connections: Vec<(usize, String)>,
    output_exec_connections: Vec<(usize, String)>,

    node_type: Box<GenericNodeType>,
}

impl NodeInstance {

    fn new(
        id: Uuid,
        input_data_connections: Vec<(usize, String)>,
        output_exec_connections: Vec<(usize, String)>,
        node_type: Box<GenericNodeType>)
    {
        NodeInstance {
            id,
            input_data_connections,
            output_exec_connections,
            node_type,
        }
    }

    fn id(&self) -> &Uuid{
        return self.id;
    }

    fn data_conn(&self, id: usize) -> Option<(usize, String)> {
        self.input_data_connections.get(id)
    }

    fn exec_conn(&self, id: usize) -> Option<(usize, String)> {
        self.output_exec_connections.get(id)
    }
}

struct GraphFunction {
    name: String,
    nodes: HashMap<Uuid, NodeInstance>,
}

impl GraphFunction {
    fn new(name: String, nodes: HashMap<Uuid, NodeInstance>) -> Option<GraphFunction> {
        let ret = GraphFunction {
            name,
            nodes,
        };

        // make sure there is exactly one node entry
        if ret.nodes_with_type("lang:entry").len() != 1 {
             return None;
        }

        // make sure all exit nodes have the same number of
        let exit_node: Option<NodeType> = None; // node to compare to
        for n in ret.nodes_with_type("lang:exit") {
            if let en = exit_node {
                if n.node_type.data_inputs() != en.data_inputs() ||
                    n.node_type.data_outputs() != en.data_outputs() ||
                    n.node_type.exec_inputs() != en.exec_inputs() ||
                    n.node_type.exec_outputs() != en.exec_outputs() {
                    return None;
                }
            }
        }
    }

    fn node(&self, id: Uuid) -> Option<&NodeInstance> {
        self.nodes.get(id)
    }

    fn nodes_with_type(&self, node_type: String) -> Vec<&NodeInstance> {
        self.nodes.iter().filter(|n: &NodeInstance| n.node_type.name() == node_type).collect()
    }
}

impl GenericNodeType for GraphFunction {
    fn data_inputs(&self) -> Vec<NamedDataType> {

    }
    fn data_outputs(&self) -> Vec<NamedDataType> {

    }

    fn exec_inputs(&self) -> Vec<String> {

    }
    fn exec_outputs(&self) -> Vec<String> {

    }
}

struct GraphModule {
    name: String,
    functions: Vec<GraphFunction>,
}

impl Module for GraphModule {
    fn name(&self) -> String {
        self.name
    }

    fn data_types(&self) -> Vec<&DataType> {
        Vec::new()
    }
    fn node_types(&self) -> Vec<&GenericNodeType> {
        let mut ret: Vec<&GenericNodeType> = Vec::new();

        for f in &self.functions {
            ret.push(&f)
        }
    }

}
