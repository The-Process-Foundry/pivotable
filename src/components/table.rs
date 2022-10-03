//! Define the layout and configuration of the table
//!
//! This contains default options and all the state values that define the display

use crate::local::*;
use std::{
  collections::{HashMap, HashSet},
  sync::{Arc, Mutex},
};

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

/// A set of custom events to modify the node on specific events
pub trait SelectableNode: Clone + fmt::Debug {
  type Error: Into<SelectorError>;

  /// Custom code to be run when a child is added
  fn added_child<T>(&mut self, child: GenericNode<Self>) -> Result<()>
  where
    T: SelectableNode,
  {
    Ok(())
  }

  /// Custom code to be run when a child is added
  fn removed_child<T>(&mut self, child: GenericNode<Self>) -> Result<()>
  where
    T: SelectableNode,
  {
    Ok(())
  }

  /// Custom code to be run when a child is added

  /// Custom code to be run when a child is added
  fn set_parent<T>(&mut self, child: GenericNode<Self>) -> Result<()>
  where
    T: SelectableNode,
  {
    Ok(())
  }
}

/// This is a fat pointer wrapping the most common edges needed to render a full table
#[derive(Clone)]
pub struct GenericNode<T>
where
  T: SelectableNode,
{
  guid: Uuid,

  ptr: Arc<Mutex<T>>,

  // An edge pointing to the parent node.
  parent: Option<Box<GenericNode<T>>>,

  // A list of pointers to the related children
  children: Vec<GenericNode<T>>,
}

impl<T> GenericNode<T>
where
  T: SelectableNode,
{
  pub fn new(value: T) -> GenericNode<T> {
    GenericNode {
      guid: Uuid::new_v4(),
      ptr: Arc::new(Mutex::new(value)),
      parent: None,
      children: Vec::new(),
    }
  }

  pub fn add_child(&mut self, child: GenericNode<T>) -> Result<()> {
    self.children.push(child.clone());
    self.ptr.lock().unwrap().added_child::<T>(child.clone())
  }

  pub fn set_parent(&mut self, parent: GenericNode<T>) {
    if self.parent.is_some() {
      panic!("Cannot change your parents, no matter how much you may wish it")
    };

    self.parent = Some(Box::new(parent.clone()));

    let mut unwrapped = parent.ptr.lock().unwrap();
    parent.add_child(self.clone());
  }
}

/// General options for setting up a table
#[derive(Debug)]
pub struct SelectorConfig {}

/// A container for the current state of the table
///
/// TODO: This should be generic. Is the type Accessible or more specific to Grapht or even a common
pub struct SelectorTable<T>
where
  T: SelectableNode,
{
  root: GenericNode<T>,
  nodes: HashMap<Uuid, GenericNode<T>>,
  selected: HashSet<Uuid>,
}

impl<T> SelectorTable<T>
where
  T: SelectableNode,
{
  pub fn new(query: &str) -> SelectorTable<T> {
    SelectorTable {
      root,
      nodes,
      selected: HashSet::new(),
    }
  }

  pub fn render_row(&self, indent: u16, node: Arc<Mutex<T>>) -> Vec<Html> {
    let mut rows = Vec::new();
    let row = node.lock().unwrap();
    rows.push(html! {
      <tr>
        <td></td>
        <td></td>
        <td></td>
        <td></td>
      </tr>
    });

    for child in &row.children {
      rows.extend(self.render_row(indent + 1, child.clone()))
    }
    rows
  }
}

impl<T> Component for SelectorTable<T>
where
  T: SelectableNode + 'static,
{
  type Message = ();
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self::new("")
  }

  fn view(&self, _ctx: &Context<Self>) -> Html {
    debug!("In the Selector Table View");

    // let td_class = classes!();

    // let data: Vec<Vec<Vec<[]>> = Vec::new();
    let multicheck = MultiCheck::new();

    let columns = vec![
      ("pretty_id", "Pretty Id"),
      ("org_name", "Org Name"),
      ("balance", "Balance"),
      ("submissions", "Submissions"),
      ("payments", "Payments"),
      ("invoices", "Invoices"),
      // Open and completed line items
      // ("deliverables", "Deliverables"),
      // ("open", ""),
      // ("completed", ""),
      // ("", ""),
      // ("", ""),
    ];

    // Create the static column headers
    let mut headers = vec![
      html! {<th classes={classes!("border", "border-neutral-100")}>{multicheck.render(None)}</th>},
    ];
    for (_, col) in columns {
      headers.push(html! {<th classes={classes!("border", "border-neutral-100")}>{col}</th>});
    }

    let mut rows: Vec<Html> = Vec::new();
    let locked = &self.root.lock().unwrap();
    info!("Rendering the children of root: {}", locked.guid);
    for child in &locked.children {
      info!("Making a child row");
      rows.extend(self.render_row(0, child.clone()))
    }

    html!(
      <div class={classes!("border", "border-neutral-100", "p-3", "bg-white")}>
        <div>
          <table class={classes!("w-full")}>
            <thead>
              <tr>
                {headers}
              </tr>
            </thead>
            <tbody>
              {rows}
            </tbody>
          </table>

        </div>
      </div>
    )
  }
}

/// General properties that are distributed among the the header, body, and footer of the table
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct SelectorTableProps {
  /// Unique Identifier for Selector
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SelectorTableEvent {}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct TableHeaderProps {
  /// Unique Identifier for TableHeader
  #[prop_or_default]
  pub id: String,

  /// CSS Classes used for rendering
  #[prop_or_default]
  pub class: Classes,
}

/// TableHeader
#[derive(Debug)]
pub struct TableHeader {}

impl Component for TableHeader {
  type Message = TableHeaderMsg;
  type Properties = TableHeaderProps;

  fn create(_ctx: &Context<TableHeader>) -> Self {
    TableHeader {}
  }

  fn view(&self, ctx: &Context<TableHeader>) -> Html {
    debug!("Rendering view for TableHeader");

    html! {
      <thead class={ctx.props().class.clone()}>
        {"TableHeader: view has not been implemented"}
      </thead>
    }
  }
}

/// Messages to be delivered to the table header
#[derive(Clone, Debug, PartialEq)]
pub enum TableHeaderMsg {}

#[derive(Debug, Clone)]
struct Header {
  // The column's start position
  start: u16,
  // How many columns the header contains (colspan)
  width: u16,
  // How many parent ancestors it has (rowspan)
  depth: u16,
  // The text
  value: String,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Columns {
  columns: Vec<Column>,
}

impl Columns {
  pub fn new() -> Columns {
    Columns {
      columns: Vec::new(),
    }
  }

  pub fn add_columns(&mut self, columns: Vec<Column>) -> Result<()> {
    self.columns.extend(columns);
    Ok(())
  }

  pub fn add_column(&mut self, column: Column) -> Result<()> {
    self.columns.push(column);
    Ok(())
  }
}

/// Allows for nesting column headers and grouping aggregates
#[derive(Debug, Clone, PartialEq)]
pub enum Column {
  /// One or more
  Group(ColumnGroup),
  Single(ColumnCell),
}

impl Column {
  pub fn new_cell(name: &str) -> Column {
    Column::Single(ColumnCell::new(name))
  }

  pub fn new_group(name: &str, children: Vec<Column>) -> Column {
    let mut group = ColumnGroup::new(name);
    for col in children {
      group.add_column(col).expect("Received a non-column")
    }
    Column::Group(group)
  }

  pub fn add_column(&mut self, column: Column) -> Result<()> {
    match self {
      Column::Group(group) => group.add_column(column),
      Column::Single(_) => Err(err!(
        InvalidVariant,
        "Cannot add columns to a single column"
      )),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnGroup {
  pub id: Uuid,
  pub name: String,
  pub children: Vec<Box<Column>>,
  // A data filter used for generating aggregates, essentially a sub-table
  // data: impl Row
}

impl ColumnGroup {
  pub fn new(name: &str) -> ColumnGroup {
    ColumnGroup {
      id: Uuid::new_v4(),
      name: name.to_string(),
      children: Vec::new(),
    }
  }

  pub fn add_column(&mut self, col: Column) -> Result<()> {
    todo!("add_column")
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnCell {
  pub id: Uuid,
  pub name: String,
  /// Allow the values in this column to be used to sort the table
  pub sorting: ColumnSortable,
  /// How to use the database to generate the value
  pub value: String,
}

impl ColumnCell {
  pub fn new(name: &str) -> ColumnCell {
    ColumnCell {
      id: Uuid::new_v4(),
      name: name.to_string(),
      sorting: ColumnSortable::Sortable,
      value: String::new(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnSortable {
  Sortable,
  Disabled,
}
