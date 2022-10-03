//! Creating a selectable table from random data
//!
//! This is going to be the actual code

pub mod components;
pub mod logger;

pub mod demo;

pub mod errors;

pub mod data;

pub mod invoicer;

/// Imported options required to make a basic Selector table
pub mod prelude {
  pub use super::{
    components::{
      Button, Column, ColumnCell, ColumnGroup, ColumnSortable, Columns, PivoTable, SelectorFooter,
      SelectorHeader, SelectorTable,
    },
    data::{DataAction, DataEvent, SelectorData},
    errors::*,
    logger::*,
  };
}

pub(crate) mod local {
  pub use super::{components::*, errors::*};
  pub use crate::{err, err_into, unwrap};

  // Make sure no-std by default
  pub use core::{any, fmt, hash, result};

  // Using Arc for the DB, for use as a thread-safe CoW
  pub use std::rc::Rc;
  pub use std::sync::Arc;

  pub use uuid::Uuid;

  pub use yew::{prelude::*, FunctionComponent};

  pub use tracing::{debug, error, info, warn};

  pub use fhl_common::prelude::DbApi;
}

// This is the entry point for the web app
fn main() {
  let log = logger::echo_to_console(None);
  let _ = tracing::subscriber::set_global_default(log);

  tracing::debug!("Running the demo table site");

  yew::Renderer::<demo::App>::new().render();
}
