//! Configurable buttons designed to alter the table display and data
//!
//!

pub mod base;
pub use base::*;

pub mod sort;

/// Large, clickable icons displayed to the right of the title
#[derive(Debug)]
pub enum IconTool {
  Sort,
  Filter,
  Search,
  Config,
  Reset,
  // Custom(Box<dyn IconTool>),
}
