//! An example being used to drive development. This will eventually be moved to the examples folder

use crate::{invoicer::fhl_prelude::*, prelude::*};
use gquery::prelude::*;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use fhl_common::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
  // Create the demo database/graph as a context
  let data = FhlGraph::init("InvoicerDemo", "redis://127.0.0.1:/")
    .expect("Could not open and initialize the FHL Graph");

  html! {
    <FhlDemo graph={data.graph()}/>
  }
}

/// Generic message container separating events from actions
#[derive(Debug, Clone, PartialEq)]
pub enum Msg<A, E> {
  Action(A),
  Event(E),
}

#[derive(Clone, Debug)]
pub enum FhlDemoMsg {
  Selector,
  DataServer(Msg<DataAction, DataEvent>),
}

impl From<DataEvent> for FhlDemoMsg {
  fn from(evt: DataEvent) -> Self {
    FhlDemoMsg::DataServer(Msg::Event(evt))
  }
}

impl From<DataAction> for FhlDemoMsg {
  fn from(act: DataAction) -> Self {
    FhlDemoMsg::DataServer(Msg::Action(act))
  }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct FhlDemoProps {
  /// Unique Identifier for FhlDemo
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,

  /// Child components
  #[prop_or_default]
  pub children: Children,

  /// The graph containing the demo data
  pub graph: Arc<Grapht>,
}

/// FhlDemo
#[derive(Debug)]
pub struct FhlDemo {
  columns: Columns,
  db: Arc<FhlDb>,
}

impl Component for FhlDemo {
  type Message = FhlDemoMsg;
  type Properties = FhlDemoProps;

  fn create(_ctx: &Context<FhlDemo>) -> Self {
    let mut columns = Columns::new();
    columns
      .add_columns(vec![Column::Single(ColumnCell::new("ID"))])
      .expect("Error adding columns");

    let mut db = FhlDb::new();

    FhlDemo {
      columns,
      db: Arc::new(db),
    }
  }

  /// The reducer for the demo
  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      FhlDemoMsg::Selector => {
        todo!("Route the selector messages")
      }
      FhlDemoMsg::DataServer(_) => todo!("No data server yet. Making the table first"), //self.db.reduce(msg),
    }
  }

  fn view(&self, _ctx: &Context<FhlDemo>) -> Html {
    debug!("Rendering view for FhlDemo");
    html! {
      <div id="SiteRoot" >
        <BootstrapDb />
        <PivoTable columns={self.columns.clone()}/>
      </div>
    }
  }

  fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
    if first_render {
      debug!("Rendered the demo for the first time");
    }
  }
}
#[derive(Clone, Debug)]
pub enum BootstrapDbMsg {}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct BootstrapDbProps {
  /// Unique Identifier for BootstrapDb
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,

  /// Child components
  #[prop_or_default]
  pub children: Children,
}

/// BootstrapDb
#[derive(Debug)]
pub struct BootstrapDb {}

impl BootstrapDb {
  pub fn query() -> crate::errors::Result<()> {
    Ok(())
  }
}

impl Component for BootstrapDb {
  type Message = BootstrapDbMsg;
  type Properties = BootstrapDbProps;

  fn create(_ctx: &Context<BootstrapDb>) -> Self {
    BootstrapDb {}
  }

  fn view(&self, _ctx: &Context<BootstrapDb>) -> Html {
    debug!("Rendering view for BootstrapDb");
    let onclick = Callback::from(move |_| {
      info!("Clicked the Reload button");
    });
    html! {
      <div class={classes!("h-10", "w-fit", "m-auto")}>
        <Button {onclick}>{"Reload DB"}</Button>
      </div>
    }
  }
}

/*
    -webkit-font-smoothing: antialiased;
    background-color: initial;
    background-image: linear-gradient(-180deg, #FF7E31, #E62C03);
    border-radius: 6px;
    color: #FFFFFF;
    cursor: pointer;
    display: inline-block;
    font-family: Inter,-apple-system,system-ui,Roboto,"Helvetica Neue",Arial,sans-serif;
    height: 40px;
    line-height: 40px;
    outline: 0;
    overflow: hidden;
    padding: 0 20px;
    pointer-events: auto;
    position: relative;
    text-align: center;
    touch-action: manipulation;
    -webkit-user-select: none;
    vertical-align: top;
    white-space: nowrap;
    width: 100%;
    z-index: 9;
    border: 0;
    transition: box-shadow .2s;
    box-shadow: rgba(253, 76, 0, 0.5) 0 3px 8px;

*/
