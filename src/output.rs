use serde::{Deserialize, Serialize};

/// An output specification for a [Net](crate::Net)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Output {
    r#type: Option<String>,
    description: Option<String>,
    from: String,
}
