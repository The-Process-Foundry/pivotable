//! Manage title, icons, and configuration for a table

use crate::local::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SelectorHeaderState {
  title: Option<String>,
}

impl SelectorHeaderState {
  pub fn new(props: &SelectorHeaderProps) -> SelectorHeaderState {
    SelectorHeaderState {
      title: props.title.map(|x| x.to_string()),
    }
  }

  fn set_title(&mut self, title: &str) {
    self.title = Some(title.to_string())
  }
}

impl Reducible for SelectorHeaderState {
  type Action = SelectorHeaderAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    self.clone()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectorHeaderEvent {}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectorHeaderAction {}

/// Items that always appear at the top of a selector
#[function_component(SelectorHeader)]
pub fn selector_header(props: &SelectorHeaderProps) -> Html {
  debug!("In the Selector Table View");

  html!(
    <div class={classes!("w-full","flex","flex-row")}>
      <div class={classes!("p-2", "inline-block", "flex-1")}>
        <SelectorTitle title={props.title.unwrap()} />
      </div>
      <div class={classes!("p-2", "inline-block")}>
        <span class={classes!("float-right")}>{"Icons Here"}</span>
        //   <SelectorIcon title={ctx.props().title} />
      </div>
    </div>
  )
}

/// Properties to configure the header
#[derive(Debug, PartialEq, Properties)]
pub struct SelectorHeaderProps {
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
  pub title: Option<&'static str>,
}

#[function_component(SelectorTitle)]
pub fn selector_title(props: &SelectorTitleProps) -> Html {
  debug!("Rendering view for SelectorTitle");
  match props.title {
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
  pub title: Option<&'static str>,
}

pub trait SelectorHeaderIcon: fmt::Debug {}

/// Large, clickable icons displayed to the right of the title
#[derive(Debug)]
pub enum SelectorIcon {
  Sort,
  Filter,
  Search,
  Config,
  Reset,
  Custom(Box<dyn SelectorHeaderIcon>),
}

pub struct SortIcon {}
