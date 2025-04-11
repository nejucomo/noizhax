use serde::{Deserialize, Serialize};

use crate::node::SerdeNode;
use crate::table::NamelessTable;
use crate::{Input, Node, Output, Table};

/// A specification of an encapsulated network of signal processing nodes
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from = "SerdeNet", into = "SerdeNet")]
pub struct Net {
    description: String,
    input: Table<Input>,
    output: Table<Output>,
    node: Table<Node>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerdeNet {
    description: String,
    input: Table<Input>,
    output: Table<Output>,
    node: NamelessTable<SerdeNode>,
}

impl From<SerdeNet> for Net {
    fn from(snet: SerdeNet) -> Self {
        Net {
            description: snet.description,
            input: snet.input,
            output: snet.output,
            node: Table::from(snet.node),
        }
    }
}

impl From<Net> for SerdeNet {
    fn from(net: Net) -> Self {
        SerdeNet {
            description: net.description,
            input: net.input,
            output: net.output,
            node: NamelessTable::from(net.node),
        }
    }
}
