//! A builder to construct a Selector table

/// Configuration for building a selector, made to be created using a macro
pub struct SelectorBuilder {}

impl SelectorBuilder {
  pub fn demo() -> SelectorBuilder {
    let yml = r#"
      title: Nested Selector
      groups:
        - label: Root Organizations
          columns:
            - type: roll_up_checkbox
            - label: Short ID
              type: data
              source: short_id
              sortable: true
            - label: Org Name
              source: name
          child:
    "#;
  }
}
