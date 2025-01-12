//!
//!
//! # Examples
//!
//! ```rust
//! extern crate serde;
//! extern crate serde_xml_rs;
//!
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use serde_xml_rs::{from_str, to_string};
//!
//! #[derive(Debug, Serialize, Deserialize, PartialEq)]
//! struct Item {
//!     name: String,
//!     source: String,
//! }
//!
//! fn main() {
//!     let src = r#"<Item><name>Banana</name><source>Store</source></Item>"#;
//!     let should_be = Item {
//!         name: "Banana".to_string(),
//!         source: "Store".to_string(),
//!     };
//!
//!     let item: Item = from_str(src).unwrap();
//!     assert_eq!(item, should_be);
//!
//!     let reserialized_item = to_string(&item).unwrap();
//!     assert_eq!(src, reserialized_item);
//! }
//! ```


#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate xml;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod error;
pub mod de;
pub mod ser;

pub use crate::error::{Error, ErrorKind};
pub use xml::reader::{EventReader, ParserConfig};
pub use crate::ser::{to_string, to_writer, Serializer};
pub use crate::de::{from_reader, from_str, Deserializer};
