//! `noisehax`: (yet another) a library / cli tool for signal processing networks
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSpec {
    description: String,
    input: Table<Input>,
    output: Table<Output>,
    node: Table<WiredNode>,
}

pub type Table<T> = BTreeMap<String, T>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    r#type: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    r#type: Option<String>,
    description: Option<String>,
    from: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WiredNode {
    r#type: Option<String>,
    input: Table<String>,
}

#[test]
fn parse_example_note() {
    dbg!(toml::from_str::<NodeSpec>(include_str!(
        "../examples/note.toml"
    )))
    .unwrap();
}
