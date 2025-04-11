use serde::{Deserialize, Serialize};

use crate::Table;

/// A node specification within a [Net](crate::Net)
#[derive(Clone, Debug)]
pub struct Node {
    r#type: String,
    input: Table<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SerdeNode {
    r#type: Option<String>,
    input: Table<String>,
}

impl<'a> From<(&'a str, SerdeNode)> for Node {
    fn from((name, sn): (&'a str, SerdeNode)) -> Self {
        Node {
            r#type: sn.r#type.unwrap_or_else(|| name.to_string()),
            input: sn.input,
        }
    }
}

impl<'a> From<(&'a str, Node)> for SerdeNode {
    fn from((name, node): (&'a str, Node)) -> Self {
        SerdeNode {
            // Omit `type` if it's equal to our name
            r#type: if node.r#type == name {
                None
            } else {
                Some(node.r#type)
            },
            input: node.input,
        }
    }
}
