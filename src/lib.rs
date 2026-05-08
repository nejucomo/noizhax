//! `noizhax`: signal processing network toy
#![deny(missing_docs)]

mod input;
mod net;
mod node;
mod output;
mod table;

pub use crate::input::Input;
pub use crate::net::Net;
pub use crate::node::Node;
pub use crate::output::Output;
pub use crate::table::Table;

#[test]
fn parse_example_note() {
    dbg!(toml::from_str::<Net>(include_str!("../examples/note.toml"))).unwrap();
}
