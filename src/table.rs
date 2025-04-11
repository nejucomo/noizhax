use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A TOML table of `T` values
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Table<T>(BTreeMap<String, T>);

/// An intermediate serialization type which lacks table name information
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NamelessTable<T>(BTreeMap<String, T>);

impl<S, T> From<NamelessTable<S>> for Table<T>
where
    T: for<'a> From<(&'a str, S)>,
{
    fn from(t: NamelessTable<S>) -> Self {
        Table(map_with_name(t.0))
    }
}

impl<S, T> From<Table<S>> for NamelessTable<T>
where
    T: for<'a> From<(&'a str, S)>,
{
    fn from(t: Table<S>) -> Self {
        NamelessTable(map_with_name(t.0))
    }
}

fn map_with_name<S, T>(src: BTreeMap<String, S>) -> BTreeMap<String, T>
where
    T: for<'a> From<(&'a str, S)>,
{
    src.into_iter()
        .map(|(name, s)| {
            let t = T::from((name.as_str(), s));
            (name, t)
        })
        .collect()
}
