use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A TOML table of `T` values
#[derive(Debug, Serialize, Deserialize)]
pub struct Table<T>(BTreeMap<String, T>);
