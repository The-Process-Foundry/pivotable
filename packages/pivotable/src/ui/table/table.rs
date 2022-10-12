//! Define the layout and configuration of a table
//!
//! The root configuration of a table.
//! This contains default options and all the state values that define the display. It is backed by
//! a Grapht DataSet in order

use crate::local::*;
use serde::{Deserialize, Serialize};

/// Messages a table can emit
#[derive(Clone, Debug)]
pub enum PivotableMsg {}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct PivotableProps {
  /// Unique Identifier for Pivotable
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,

  /// Child components
  #[prop_or_default]
  pub children: Children,

  #[prop_or_default]
  pub config: PivotableConfig,
}

/// A Pivotable
#[derive(Debug)]
pub struct Pivotable {
  // Header details
  // header: Option<HeaderConfig>
  // Footer details
  // Body details

  // The raw data making up the table
  // data: DataSet,

  // Position of the window in the dataset.
  // THINK: Is this part of the DataSet or is it the table? Leaning toward the former
  // cursor: DataCursor,
}

impl Pivotable {
  pub fn new(/*data: DataSet*/) -> Pivotable {
    Pivotable {
      // data: DataSet<G>
    }
  }
}

impl Component for Pivotable {
  type Message = PivotableMsg;
  type Properties = PivotableProps;

  fn create(_ctx: &Context<Pivotable>) -> Self {
    // Use the context to get the proper dataset ctx.props().dataset;
    Pivotable::new()
  }

  fn view(&self, ctx: &Context<Pivotable>) -> Html {
    info!("Rendering view for Pivotable");
    let conf = ctx.props().config.clone();

    let header = match conf.header {
      Some(conf) => Some(html! {
        <div id="HeaderWrapper" class={classes!("w-full")}>
          <PivotableHeader {conf} />
        </div>
      }),
      None => None,
    };

    let body = match conf.body {
      Some(conf) => Some(html! {
        <div id="BodyWrapper" class={classes!("w-full")}>
          <PivotableBody {conf} />
        </div>
      }),
      None => None,
    };

    let footer = match conf.footer {
      Some(conf) => Some(html! {
        <div id="FooterWrapper" class={classes!("w-full")}>
          <PivotableFooter {conf} />
        </div>
      }),
      None => None,
    };

    html! {
      <div class={classes!("bg-white")}>
        {header}
        {body}
        {footer}
      </div>
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct PivotableConfig {
  /// Settings for the header
  header: Option<HeaderConfig>,

  /// Settings to display the data
  body: Option<BodyConfig>,

  /// Settings for what to display at the bottom of the page
  footer: Option<FooterConfig>,
}

impl Pivotal for PivotableConfig {}
