//!
//! The program output template resource GET request.
//!

use serde_derive::Deserialize;

///
/// The program output template resource GET query.
///
#[derive(Debug, Deserialize)]
pub struct Query {
    /// The name of the requested program.
    pub name: String,
    /// The name of the requested entry within the program.
    pub entry: String,
}
