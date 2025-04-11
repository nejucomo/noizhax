use serde::{Deserialize, Serialize};

use crate::Table;

/// A node specification within a [Net](crate::Net)
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    r#type: Option<String>,
    input: Table<String>,
}
