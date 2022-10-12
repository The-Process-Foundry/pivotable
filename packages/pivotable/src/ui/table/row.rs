//! A generic data row

use crate::local::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// Properties to configure the header
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct RowProps {
  /// Unique Identifier for SelectorTitle
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,

  /// Child components
  #[prop_or_default]
  pub children: Children,

  /// The title to be displayed. None means it will render a null
  #[prop_or_default]
  pub conf: RowBuilder,
}

#[function_component(Row)]
pub fn row(props: &RowProps) -> Html {
  html! {}
}

#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct RowConfig {
  /// An identity to be able to reuse the same row layout
  name: String,

  /// A function to get/generate a new unique identifier for each row
  key: Option<String>,

  /// A list of columns
  columns: Vec<ColumnConfig>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RowBuilder {
  config: RowConfig,

  /// All the columns that are available
  columns: HashMap<String, Arc<ColumnBuilder>>,

  /// An ordered list of all the visible columns
  shown: Vec<Arc<ColumnBuilder>>,
}

impl RowBuilder {
  pub fn new(config: RowConfig) -> RowBuilder {
    RowBuilder {
      config,
      columns: HashMap::new(),
      shown: Vec::new(),
    }
  }

  /// Use the builders to generate the header row
  pub fn render_header_row(&self) -> Html {
    let mut cells = Vec::new();
    for cell in &self.shown {
      cells.push(cell.render_header())
    }

    html! {
      <div id="HeaderRow" classes={classes!("flex", "flex-row")}>
        {cells}
      </div>
    }
  }

  // TODO: Column management
  // - Change column visibility
  // - Change column order
  // - Change column width

  pub fn add_column(&mut self, column: ColumnConfig) -> Result<()> {
    // Validate/add group
    // build

    todo!("Row::add_column")
  }

  /// Toggle a column between
  pub fn toggle_visibility(&mut self, _column: &str) -> Result<()> {
    todo!("toggle_visibility is waiting on where the rendering info triggered from")
  }
}

impl From<RowConfig> for RowBuilder {
  fn from(config: RowConfig) -> RowBuilder {
    info!("Initialializing a BodyBuilder from config:\n{:?}", config);

    let mut builder = RowBuilder::new(config.clone());

    // Add any groups

    // Add the columns
    for column in config.columns {
      builder
        .add_column(column)
        .expect("Tried to add an invalid column")
    }

    builder
  }
}

#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct ColumnGroup {
  members: HashMap<String, ColumnConfig>,
}

/// Create a unique identifier for consistently generating row guids
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum GetKey {
  /// No way to generate a key. Will create a random guid for each new row
  Random,

  /// Use the get_guid function on the given node during rendering (Default)
  Inherit,
  // /// Use the information stored in the builder and node to generate a new guid
  // Custom(Fn(RowBuilder, Node) -> Uuid),
}

impl Default for GetKey {
  fn default() -> GetKey {
    GetKey::Inherit
  }
}
