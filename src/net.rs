use serde::{Deserialize, Serialize};

use crate::{Input, Node, Output, Table};

/// A specification of an encapsulated network of signal processing nodes
#[derive(Debug, Serialize, Deserialize)]
pub struct Net {
    description: String,
    input: Table<Input>,
    output: Table<Output>,
    node: Table<Node>,
}
