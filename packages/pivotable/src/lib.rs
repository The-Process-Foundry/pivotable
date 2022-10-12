//! Creating a selectable table from random data
//!
//! This is going to be the actual code

// An easy way to define, configure, and construct a Pivotable
pub mod builder;

// All the visual elements, components, and panels used
pub mod ui;

// Print errors to the console and other logging tools.
pub mod logger;

// Custom errors for the app
pub mod errors;

// Interacting with the data set for creating forms
// pub mod data;

/// Imported options required to make a basic Selector table
pub mod prelude {
  pub use super::{
    builder::*,
    // data::{DataAction, DataEvent, SelectorData},
    errors::*,
    logger::*,
    ui::{styler::*, table::*, tools::*},
  };
  pub use crate::classmap;
}

pub(crate) mod local {
  pub use super::{builder::*, errors::*, ui::*};
  pub use crate::{err, err_into, unwrap};

  // Make sure no-std by default
  pub use core::{any, fmt, hash, result};

  // Using Arc for the DB, for use as a thread-safe Copy on Write (CoW)
  // This can use Tokio, which is why I'm putting it in here
  pub use std::{rc::Rc, sync::Arc};

  pub use uuid::Uuid;

  pub use yew::{prelude::*, FunctionComponent};

  pub use tracing::{debug, error, info, warn};
}
