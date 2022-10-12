//! Change element styles based on state
//!
//! A common item is to change presentation based on the state of remote items. This is meant to
//! allow

use crate::local::*;
use serde::{Deserialize, Serialize};

use std::{
  collections::{HashMap, HashSet},
  iter::FromIterator,
};

#[macro_export]
/// Use a simple text list to generate a full class map
macro_rules! classmap(
  ($($val:expr$(,)?)+) => {
    ClassMap::new(vec![$($val.to_string()),+])
  }
);

/// A more complicated method of building classes, allowing toggles and dynamic rules
///
/// This is based on React classnames: https://www.npmjs.com/package/classnames
/// TODO: Make this an independent crate
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassMap {
  // /// The default set of classes
  // base: Classes,
  /// Classes to toggle when the pseudoclass is active
  ///
  /// The empty string is the base classes.
  classes: HashMap<String, HashSet<String>>,
}

impl ClassMap {
  /// Creates a new classmap for the default
  pub fn new(classes: Vec<String>) -> ClassMap {
    let mut map = HashMap::new();
    map.insert("".to_string(), HashSet::from_iter(classes));
    ClassMap { classes: map }
  }

  pub fn set_state() -> Classes {
    todo!()
  }
}

impl From<ClassMap> for Classes {
  fn from(map: ClassMap) -> Classes {
    let mut classes = Classes::new();
    let _ = map.classes.get("").map(|base| {
      for cls in base.iter() {
        classes.push(cls.clone())
      }
    });

    for (key, value) in map.classes.iter() {
      if key.is_empty() {
        continue;
      };
      for cls in value.iter() {
        classes.push(format!("{}:{}", key, cls))
      }
    }

    classes
  }
}
