//! A permanently displayed div shown at the bottom of a table
//!
//! This usually has the submit/clear buttons if the Pivotable is a form

use crate::local::*;
use serde::{Deserialize, Serialize};

/// Define how to render the body
#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct FooterConfig {}

#[function_component(PivotableFooter)]
pub fn pivotable_footer(props: &PivotableFooterProps) -> Html {
  debug!("In the Pivotable footer view");

  html! {
    <div class={classes!("w-full","flex","flex-col")}>
      <div class={classes!("p-2", "text-left")}>
        {"Footer Count goes here"}
      </div>
      <div>
        {"Footer Buttons go here"}
      </div>
    </div>
  }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PivotableFooterState {}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PivotableFooterProps {
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
  pub conf: FooterConfig,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PivotableFooterEvent {}
