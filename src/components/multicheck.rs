//! State and display of a set of related checkboxes
//!
//! This is designed to generically combine state from many checkboxes into one.

use crate::local::*;
use yew::prelude::*;

use std::collections::HashMap;

/// How to configure/render
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MultiCheckProps {
  /// Unique Identifier for Selector
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,

  /// Child components
  #[prop_or_default]
  pub children: Children,
}

#[derive(Clone, Debug)]
pub enum CheckState {
  /// All available children (including self) are selected
  On,

  /// Some children are
  Partial,
  Off,
}

#[derive(Clone, Debug)]
pub enum MultiCheckMsg {
  /// Turn all the available children on
  AllOn,

  /// Turn off all the children
  AllOff,

  /// Turn on all the given children
  ListOn(Vec<Uuid>),

  /// Turn off all the listed children
  ListOff(Vec<Uuid>),

  /// Switch a single item
  Toggle(Uuid),
}

/// MultiCheck
///
/// This is designed to be a controller.
#[derive(Clone, Debug)]
pub struct MultiCheck {
  /// The grand total of child options that are selectable by the checkbox.
  count: u32,

  /// The grand total of options that are selectable by the checkbox. If none, it will display "???"
  total: Option<u32>,
}

impl MultiCheck {
  pub fn new() -> MultiCheck {
    MultiCheck {
      count: 0,
      total: None,
    }
  }

  pub fn render(&self, uuid: Option<Uuid>) -> Html {
    match uuid {
      None => {
        html! {<MultiCheckbox show_count={true} is_leaf={false} />}
      }
      Some(_uuid) => todo!("Cannot render a child checkbox yet"),
    }
  }
}

/// Implements the root checkbox
impl Component for MultiCheck {
  type Message = MultiCheckMsg;
  type Properties = MultiCheckProps;

  fn create(_ctx: &Context<MultiCheck>) -> Self {
    MultiCheck::new()
  }

  fn view(&self, ctx: &Context<MultiCheck>) -> Html {
    info!("Rendering view for MultiCheck");
    let props = ctx.props().clone();
    html! {
      <div class={classes!()}>
        {self.render(None)}
      </div>
    }
  }
}

/// How to configure/render
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MultiCheckboxProps {
  /// Unique Identifier for Selector
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,

  /// Child components
  #[prop_or_default]
  pub children: Children,

  #[prop_or_default]
  pub show_count: bool,

  /// Toggles whether
  #[prop_or_default]
  pub is_leaf: bool,

  /// The indent level for this particular box
  #[prop_or_default]
  pub indent: u32,
}

#[function_component(MultiCheckbox)]
pub fn multi_checkbox(props: &MultiCheckboxProps) -> Html {
  html! {
    <input type="checkbox" id={props.id.clone()} />
  }
}

