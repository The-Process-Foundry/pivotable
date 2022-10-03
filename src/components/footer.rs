//! A permanently displayed div shown at the bottom of a table
//!
//! This usually has the submit/clear buttons if the selector is a form

use crate::local::*;

#[function_component(SelectorFooter)]
pub fn selector_footer(props: &SelectorFooterProps) -> Html {
  debug!("In the selector footer view");

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
pub struct SelectorFooterState {}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SelectorFooterProps {}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectorFooterEvent {}
