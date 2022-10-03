//! The organization node type

use crate::{local::*, prelude::*};

use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use rust_decimal::prelude::*;
use uuid::Uuid;

/// A temporary test node for defining rows. This will be Grapht eventually
#[derive(Clone, Debug)]
pub struct Organization {
  pub guid: Uuid,
  pub pretty_id: String,
  pub org_name: String,
  pub balance: Decimal,
  // Edge Type:
  pub parent: Option<Arc<Mutex<Organization>>>,
  pub children: Vec<Arc<Mutex<Organization>>>,
}

impl Organization {
  pub fn new(pretty_id: &str, org_name: &str, balance: Decimal) -> Organization {
    Organization {
      guid: Uuid::new_v4(),
      pretty_id: pretty_id.to_string(),
      org_name: org_name.to_string(),
      balance,
      parent: None,
      children: Vec::new(),
    }
  }

  //---  This will be the GraphtNode Trait, but not going to worry about that yet

  fn get_id(&self) -> Uuid {
    self.guid.clone()
  }
}

// /// This describes how to turn this item to/from a node, payload, and edges
// impl GraphtNode for Organization {

//   /// Makes a OpenCypher compliant create statement of the item, and related nodes children.
//   ///
//   /// To make this recursive (upon options being added), we use a simple set of refs to pass around
//   fn to_node(&mut self, ) -> Result<()> {
//     let refs = refs.unwrap_or_else(|| HashMap::from(vec![("count", 0)]));
//     let mut count = refs.get("count").map(|val| );
//     let mut query = format!("CREATE (Org{}:__Organization)", count);
//     // Get the parent if not created
//     match refs.get("ParentId") {
//       Some(parent_id) => query.extend(""),
//     }

//     todo!("Create an organization")
//   }

//   fn to_graph() -> Result<String> {
//     todo!("Have not made Organization::to_node")
//   }

//   fn from_graph() -> Result<Organization> {
//     todo!("Have not made Organization::from_node")
//   }
// }
