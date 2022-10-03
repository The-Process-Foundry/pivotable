//! Components which can assemble a table

mod selector;
pub use selector::*;

// mod data;

mod table;
pub use table::*;

mod header;
pub use header::*;

mod footer;
pub use footer::*;

mod button;
pub use button::*;

mod multicheck;
pub use multicheck::*;

mod local {
  pub use super::*;
  pub use crate::local::*;
}
