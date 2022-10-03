//! Models used for my FHL demo. These are temporary and will be moved back to the FHL project

mod organization;
pub use organization::Organization;

mod graph;
pub use graph::{FhlEdge, FhlGraph, FhlNode};

pub mod fhl_prelude {
  pub use super::*;
}
