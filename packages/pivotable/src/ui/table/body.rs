//! The data display for the table

use crate::local::*;
use serde::{Deserialize, Serialize};

/// Define how to render the body
#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct BodyConfig {
  /// A pretty name to identify this body/config
  options: BodyOptions,

  /// A list of all the available columns to place in a row
  row: RowConfig,
}

impl BodyConfig {
  pub fn new() -> BodyConfig {
    BodyConfig::default()
  }
}

/// Toggle some common behaviours for the table
#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct BodyOptions {
  /// When the column headers should be frozen at the top
  headers: HeaderLocation,

  /// Number of frozen rows to keep at the top of the data box when viewing nested data.
  ///
  ///
  /// When N = 0, the parent record will be pushed to the top of the screen and scroll off
  /// immediately after
  /// When N = 1, the direct parent will always be pinned to the top and the rest will scroll off
  /// When N = 2, The parent and grandparent will always be pinned
  /// When N = 3, The root ancestor will be shown, then ... and then the parent and grandparent
  ///   (First, ..., N - 2 ,N - 1, N).
  /// When N > 3, Keep pinning ancestors until it reaches the root.
  nested_freeze: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HeaderLocation {
  /// Keep the headers visible at the top (Default)
  PinnedTop,

  /// Place the headers at the top but allow them to scroll off as the cursor moves down
  Top,

  /// Keep the headers at the foot of the table
  PinnedBottom,

  /// Do not show any column headers
  None,
}

impl Default for HeaderLocation {
  fn default() -> HeaderLocation {
    HeaderLocation::PinnedTop
  }
}

/// Properties to configure the header
#[derive(Debug, PartialEq, Properties)]
pub struct PivotableBodyProps {
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
  pub conf: BodyConfig,
}

#[function_component(PivotableBody)]
pub fn pivotable(props: &PivotableBodyProps) -> Html {
  // Full config
  let builder = BodyBuilder::from(props.conf.clone());

  // Place the column headers in either pinned location or at the top of the data
  let (pinned_top, pinned_bottom, scrolled_headers) = builder.get_header_row();
  html! {
    <div class={classes!{"w-full"}}>
      {pinned_top}
      <div id="data_rows" class={classes!{"w-full"}}>
        {scrolled_headers}
        {"I'm the data"}
      </div>
      {pinned_bottom}
    </div>
  }
}

/// Maintain the state of the body and pre-compile some models based on the config
///
/// This binds disparate parts together (such as making similar rows).
/// TODO: Serialize this as a config.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BodyBuilder {
  config: BodyConfig,

  row_builder: RowBuilder,

  /// The row containing the column headers.
  ///
  /// This is used so we can quickly access the width of
  header: Html,
}

impl BodyBuilder {
  /// Create an empty BodyBuilder
  pub fn new() -> BodyBuilder {
    let config = BodyConfig::new();

    let row_builder = RowBuilder::new(config.row.clone());
    let header = row_builder.render_header_row();

    BodyBuilder {
      config,
      row_builder,
      header,
    }
  }

  pub fn set_config(&mut self, config: &BodyConfig) -> Result<()> {
    self.row_builder = RowBuilder::new(config.row.clone());
    self.header = self.row_builder.render_header_row();
    self.config = config.clone();

    Ok(())
  }

  /// Calculates and generates a blank row layout
  ///
  /// This
  pub fn row(&self) {}

  /// Returns the headers in a tuple of possible locations: (pinned_top, pinned_bottom, scrolled)
  pub fn get_header_row(&self) -> (Html, Html, Html) {
    debug!(
      "Getting the header row in position: {:?}",
      self.config.options.headers
    );
    let mut pinned_top = html! {};
    let mut pinned_bottom = html! {};
    let mut scrolled = html! {};
    match self.config.options.headers {
      HeaderLocation::PinnedTop => pinned_top = self.header.clone(),
      HeaderLocation::PinnedBottom => pinned_bottom = self.header.clone(),
      HeaderLocation::Top => scrolled = self.header.clone(),
      HeaderLocation::None => (),
    }
    (pinned_top, pinned_bottom, scrolled)
  }
}

impl From<BodyConfig> for BodyBuilder {
  fn from(config: BodyConfig) -> BodyBuilder {
    info!("Initialializing a BodyBuilder from config:\n{:?}", config);
    let mut builder = BodyBuilder::new();
    builder.set_config(&config).expect(&format!(
      "Invalid config passed to the body builder:\n{:?}",
      config
    ));
    builder
  }
}
