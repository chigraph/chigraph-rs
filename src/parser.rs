use std::collections::HashMap;

use graph::{GraphModule, GraphFunction, NodeInstance};

use serde_json::{Error, from_str};

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct JSONModule {
    dependencies: Vec<String>,
    graphs: Vec<JSONGraph>,
    types: HashMap<String, Vec<HashMap<String, String>>>,
}

#[derive(Serialize, Deserialize)]
struct JSONGraph {
    #[serde(rename = "type")]
    type_: String,
    name: String,
    description: String,
    local_variables: HashMap<String, String>,
    data_inputs: Vec<HashMap<String, String>>,
    data_outputs: Vec<HashMap<String, String>>,
    exec_inputs: Vec<String>,
    exec_outputs: Vec<String>,
    nodes: HashMap<String, JSONNode>,
    connections: Vec<JSONConnection>,
}

#[derive(Serialize, Deserialize)]
struct JSONNode {
    #[serde(rename = "type")]
    type_: String,
    location: [f32; 2],
    data: String,
}

#[derive(Serialize, Deserialize)]
struct JSONConnection {
    #[serde(rename = "type")]
    type_: String,
    input: (String, f32),
    output: (String, f32),
}

fn parse(input: &str) -> Result<GraphModule, Error> {
    let j_mod = from_str(input)?;

    let mut funcs = Vec::<GraphFunction>::new();

    for (func_name, func) in &j_mod.graphs {
        let mut nodes = Vec::<NodeInstance>::new();

        for (node_id, node) in &f.nodes {
            nodes.push(NodeInstance::new(match Uuid::parse_str(node_id) {
                Ok(i) => i,
                Err(e) => return Err(Error::custom(format!("{:?}", e))),
            }, vec!(), vec!(), ))
        }
    }
}
