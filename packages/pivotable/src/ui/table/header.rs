//! Manage title, icons, and configuration for a table

use crate::local::*;

use serde::{Deserialize, Serialize};

// pub use std::collections::HashSet;

/// Items that always appear at the top of a selector
///
/// This is usually a title
#[function_component(PivotableHeader)]
pub fn selector_header(props: &PivotableHeaderProps) -> Html {
  debug!("In the Selector Table View");

  html!(
    <div class={classes!("w-full","flex","flex-row")}>
      <SelectorTitle title={props.conf.title.clone()} />
      <div class={classes!("p-2", "inline-block")}>
        <span class={classes!("float-right")}>{"Icons Here"}</span>
        //   <SelectorIcon title={ctx.props().title} />
      </div>
    </div>
  )
}

/// Define the headers
#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct HeaderConfig {
  title: Option<String>,
  help: Option<String>,
}

impl Pivotal for HeaderConfig {}

/// Properties to configure the header
#[derive(Debug, PartialEq, Properties)]
pub struct PivotableHeaderProps {
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
  pub conf: HeaderConfig,
}

#[function_component(SelectorTitle)]
pub fn selector_title(props: &SelectorTitleProps) -> Html {
  debug!("Rendering view for SelectorTitle");
  match &props.title {
    Some(title) => html! {
      <div class={classes!("inline-block", "text-left")}>
        <p class={props.class.clone()}>
          {title}
        </p>
      </div>
    },
    None => html! {},
  }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct SelectorTitleProps {
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
  pub title: Option<String>,
}

pub trait PivotableHeaderIcon: fmt::Debug {}

/*
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PivotableHeaderState {
  title: Option<String>,
  description: Option<String>,
  // tools: Option<HashSet<ToolIcon>>,
}

impl PivotableHeaderState {
  pub fn new(props: &PivotableHeaderProps) -> PivotableHeaderState {
    PivotableHeaderState {
      // active: Tool
    }
  }

  fn set_title(&mut self, title: &str) {
    self.title = Some(title.to_string())
  }
}

impl Reducible for PivotableHeaderState {
  type Action = PivotableHeaderAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    self.clone()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PivotableHeaderEvent {}

#[derive(Clone, Debug, PartialEq)]
pub enum PivotableHeaderAction {}
*/
