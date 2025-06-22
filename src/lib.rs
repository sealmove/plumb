use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Node {
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct Pipeline {
    pub node: HashMap<String, Node>,
}
