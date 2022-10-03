//! A selector table to bind the display together

use super::local::*;
use gquery::prelude::*;

use std::collections::HashMap;

/// A full set of events that can be triggered by the selector
#[derive(Clone, Debug, PartialEq)]
pub enum SelectorEvent {
  Header(SelectorHeaderEvent),
  Table(SelectorTableEvent),
  Footer(SelectorFooterEvent),
}

/// Facade to build a selector using defaults layouts
///
/// These props are passed down to
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct SelectorProps {
  /// Unique Identifier for Selector
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,

  /// Child components
  #[prop_or_default]
  pub children: Children,

  /// An item to show in the title bar
  #[prop_or_default]
  pub title: Option<&'static str>,

  // /// The base data query to display
  // pub source: Arc<SelectorData>,
  /// Column Descriptors
  pub columns: Columns,

  /// The data source
  ///
  /// THINK: Grapht is going to be the backend regardless, but does this create crate leakage?
  pub graph: Arc<Grapht>,
}

/// A selector table builder
///
/// This binds together all the component reducers using sane defaults. This takes the props for
/// initialization of the state and then maintains it.
#[function_component(PivoTable)]
pub fn pivotable(props: &SelectorProps) -> Html {
  let title = props.title.unwrap_or_else(|| "Selector Table");

  let ctx = use_memo(|_| SelectorContext::new(), ());
  use_effect_with_deps(
    move |_| {
      debug!("Loading the Database as an effect");
      || ()
    },
    (),
  );

  html! {
    <div class={classes!("m-auto", "m-4", "rounded-sm", "bg-neutral-100")}>
      <ContextProvider<Rc<SelectorContext>> context={ctx}>
        <SelectorHeader title={Some(title)} />
        <SelectorTable />
        <SelectorFooter />
      </ContextProvider<Rc<SelectorContext>>>
    </div>
  }
}

/// Items that need to be available to every part of the component
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SelectorContext {
  /// Items registered to receive updates
  dispatchers: HashMap<Uuid, Dispatcher>,
}

/// Wrappers for different components which can register themselves to be notified of various events
#[derive(Clone, Debug, PartialEq)]
pub enum Dispatcher {
  SelectorHeader(Uuid, UseReducerDispatcher<SelectorHeaderState>),
  Table(SelectorTableEvent),
  Footer(SelectorFooterEvent),
}

impl SelectorContext {
  pub fn new() -> SelectorContext {
    SelectorContext {
      dispatchers: HashMap::new(),
    }
  }

  /// Receives events and routes them to one or more reducers
  pub fn dispatch(&self, evt: SelectorEvent) {
    debug!("Got a message: {:?}", evt)
  }
}

/// Restrictions on how many items should be selectable
pub struct SelectorGranularity {
  min: Option<u16>,
  max: Option<u64>,
  // depth: Option<Box<SelectorGranularity>>,
}

impl SelectorGranularity {
  pub fn new() -> SelectorGranularity {
    SelectorGranularity {
      min: Some(1),
      max: None,
    }
  }
}
