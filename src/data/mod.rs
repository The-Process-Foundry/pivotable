//! Data manipulation classes
//!
//! This is used to parse and aggregate data used to build the table
//! TODO: Figure out where this should live (Wrapi?)

use crate::local::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SelectorData {
  /// A function to send out data messages
  emitter: Callback<DataEvent>,

  /// A function that maps to
  receiver: Callback<DataAction>,
}

impl SelectorData {
  pub fn new<A, E>(db: impl SelectorDataBackend<Action = A, Event = E>)
  where
    A: From<DataAction>,
    E: Into<DataEvent>,
  {
    todo!()
  }
}

/// An interface that can be used by the table to build, maintain, and query the data displayed
pub trait SelectorDataBackend {
  /// Emissions notifying the app about changes in the data
  type Event: Into<DataEvent>;

  /// Commands to query and minipulate the data
  type Action: Into<DataAction>;

  /// Initialize the data for a selector table
  ///
  /// This builds out the indices for searching, sorting, and filtering. It will also begin
  fn init(&mut self, dispatch: Callback<DataEvent>, query: &str);

  fn get_runner(&'static self) -> Callback<DataAction>;

  /// Update the internal state based on the message
  fn run(&self, act: DataAction);

  fn sort(&mut self, order: &str);
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataMsg {
  Action(DataAction),
  Event(DataEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataEvent {
  /// A hash set of pointers to records received. Just giving a count for the moment
  ReceivedRecords,
  StartedQuery,
  UpdatedQuery,
  FinishedQuery,
  StartedSort,
  FinishedSort,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataAction {
  Clear,
  Query(String),
  Sort,
  Filter,
  SetCursor,
}

/*
/// Connection to the data server
///
/// Communications are built around using Javascript Fetch
#[derive(Debug, Properties, Clone)]
pub struct DataServer<T>
where
  T: SelectorData,
{
  /// The location of the remote data source
  url: String,
  /// A queryable database that is used to build data sets for the selector
  db: Arc<T>,
}

impl<T> DataServer<T>
where
  T: DbApi + fmt::Debug,
{
  pub fn new(url: &str, db: T) -> DataServer<T> {
    DataServer {
      url: url.to_string(),
      db: Arc::new(db),
    }
  }

  pub fn reduce(&self, msg: )
}

impl<T> PartialEq for DataServer<T>
where
  T: DbApi + fmt::Debug,
{
  fn ne(&self, other: &Self) -> bool {
    !(self.url == other.url)
  }

  fn eq(&self, other: &Self) -> bool {
    self.url == other.url
  }
}
 */
