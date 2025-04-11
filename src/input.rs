use serde::{Deserialize, Serialize};

/// An input specification for a [Net](crate::Net)
#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    r#type: String,
    description: Option<String>,
}
