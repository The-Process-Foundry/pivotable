//! Implementation of the DbServer using a Grapht cache

use super::*;
// use crate::local::*;

use fhl_common::database::{DbAction, DbApi, DbEvent, DbValue, NamespaceId};

// use serde::{Deserialize, Serialize};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen_futures::JsFuture;
// use web_sys::{Request, RequestInit, RequestMode, Response};

/// Generic backend implementation for DbApi (Grapht)
impl<T, U, V> SelectorDataBackend for Arc<T>
where
  T: DbApi<Namespace = U, Instance = V>,
  U: NamespaceId,
  V: DbValue,
  Self: 'static,
{
  type Event = DbEvent;

  type Action = DbAction;

  fn init(&mut self, dispatch: Callback<DataEvent>, query: &str) {
    todo!()
  }

  fn get_runner(&'static self) -> Callback<DataAction> {
    Callback::from(move |act: DataAction| self.run(act.into()))
  }

  fn run(&self, act: DataAction) {
    todo!()
  }

  fn sort(&mut self, order: &str) {
    todo!()
  }
}

impl From<DbEvent> for DataEvent {
  fn from(msg: DbEvent) -> Self {
    match msg {
      DbEvent::ReceivedRecords => DataEvent::ReceivedRecords,
      DbEvent::StartedQuery => DataEvent::StartedQuery,
      DbEvent::UpdatedQuery => DataEvent::UpdatedQuery,
      DbEvent::FinishedQuery => DataEvent::FinishedQuery,
      DbEvent::StartedSort => DataEvent::StartedSort,
      DbEvent::FinishedSort => DataEvent::FinishedSort,
    }
  }
}

impl From<DbAction> for DataAction {
  fn from(act: DbAction) -> Self {
    match act {
      DbAction::Clear => DataAction::Clear,
      DbAction::Query(String) => DataAction::Query(String),
      DbAction::Sort => DataAction::Sort,
      DbAction::Filter => DataAction::Filter,
      DbAction::SetCursor => DataAction::SetCursor,
    }
  }
}
