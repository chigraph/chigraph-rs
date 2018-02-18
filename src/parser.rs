use serde_json::{Value, Error};

#[derive(Serialize, Deserialize)]
struct JSONModule {
    dependencies: Vec<String>,
    graphs: Vec<JSONGraph>,
    types: Map<String, Vec<Map<String, String>>>,
}

#[derive(Serialize, Deserialize)]
struct JSONGraph {
    #[serde(rename = "type")]
    type_: String,
    name: String,
    description: String,
    local_variables: Map<String, DataType>,
    data_inputs: Vec<NamedDataType>,
    data_outputs: Vec<NamedDataType>,
    exec_inputs: Vec<String>,
    exec_outputs: Vec<String>,
    nodes: Map<String, JSONNode>,
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

