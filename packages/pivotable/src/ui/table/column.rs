//! State and configuration information for a column

use crate::local::*;
use serde::{Deserialize, Serialize};

pub use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct ColumnConfig {
  /// An identity to be able to reuse the same layout
  name: String,

  /// A unique identifier for the column
  key: Option<String>,

  format: Option<String>,

  /// The contents and format of the header cell. Defaults to using a bold name
  header: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Properties)]
pub struct ColumnBuilder {
  config: ColumnConfig,
  display: Display,
}

impl ColumnBuilder {
  pub fn new(config: ColumnConfig) -> ColumnBuilder {
    ColumnBuilder {
      config,
      display: Display::default(),
    }
  }

  /// Render a cell using the header formatting
  pub fn render_header(&self) -> Html {
    todo!("Column::render_header")
  }

  /// Render a cell using the value formatting
  pub fn render_value(&self, _value: &str) -> Html {
    todo!("Column::render_value")
  }
}

impl From<ColumnConfig> for ColumnBuilder {
  fn from(config: ColumnConfig) -> Self {
    ColumnBuilder {
      config,
      display: Display::default(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Display {
  Visible,
  Hidden,
}

impl Default for Display {
  fn default() -> Display {
    Display::Visible
  }
}
