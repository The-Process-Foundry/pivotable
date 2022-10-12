//! A builder to construct a Selector table
//!
//! TODO:
//! - Convert this to a macro so any syntax errors can be highlighted
//!
//! THINK:
//! - What items need to be saved in the "cursor" that can be accessed by the renderer
//! - Templating for "Merged" columns. Such as how to "Sort"
//! - Behaviour of parent rows for pivots
//!   - base: Clone the config of the same name
//! - Builder traits that can be overridden by the user:
//!   - render: A template that takes the cursor/row
//!   - classmap: Override pseudo-classes

use crate::local::*;

use serde::{Deserialize, Serialize};

/// Tag configuration objects and add some default functionality
pub trait Pivotal: Clone + fmt::Debug + Properties + Serialize + for<'a> Deserialize<'a> {
  // /// Bind the UI component to the configuration
  // type Component: yew::Component;

  // fn render(&self, data: DataSet<G>) => html!{ <Self::Component /> }
}
