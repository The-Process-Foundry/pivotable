//! Graph data stored in Redis

use super::fhl_prelude::*;
use crate::local::*;

use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use gquery::prelude::*;
use grapht::backends::redis_graph::*;

/// Wrappers for the various node entity types that FHL knows about
pub enum FhlNode {
  Organization(Organization),
  Submission,
  Payment,
  Deliverable,
  Invoice,
}

impl GraphtNode for FhlNode {
  fn get_guid(&self) -> Uuid {
    match &self {
      FhlNode::Organization(org) => org.get_guid(),
      _ => todo!("Still need to do the other FhlNode types: {}", self),
    }
    .to_string()
  }

  fn get_type_label(&self) -> String {
    match self {
      FhlNode::Organization(_) => "Organization",
      FhlNode::Submission => "",
      FhlNode::Payment => "",
      FhlNode::Deliverable => "",
      FhlNode::Invoice => "",
    }
    .to_string()
  }
}

/// Wrappers for the various edge types that FHL knows about and included payload
pub enum FhlEdge {
  /// Pointer to the parent organization. Null is allowed
  OrganizationParent(organization::Organization, organization::Organization),

  /// Pointer to one of the children of the organization
  OrganizationChild(organization::Organization, organization::Organization),

  /// A payment made to an organization
  OrganizationPayment,

  /// A billable unit of work
  OrganizationDeliverable,

  /// A set of items Payments, Deliverables, and Unpaid Invoices for a given organization
  OrganizationInvoice,
}

pub struct FhlGraph {
  name: String,
  url: String,
  db: Arc<Grapht>,
}

impl FhlGraph {
  pub fn init(name: &str, url: &str) -> Result<FhlGraph> {
    let mut graph = Grapht::new();

    // Set up the graph if it doesn't exist
    match graph.get_backend(name) {
      Err(err) => {
        if err.is(gquery::errors::Kind::NotFound) {
          info!("Creating new Redis backend");
          let backend = RedisGraph::new(RedisGraphConfig::new(name, url));
          graph
            .add_backend(name, BackendImpl::RedisGraph(backend))
            .expect("Failed to add backend");
        } else {
          panic!(
            "TypeMismatch: Expected a RedisGraph backend, but received {:?}",
            err
          )
        }
      }
      Ok(backend) => {
        if let BackendImpl::RedisGraph(_) = &*backend.read().unwrap() {
          info!("Graph already created, checking connection");
        } else {
          panic!(
            "TypeMismatch: Expected a RedisGraph backend, but received {:?}",
            backend
          )
        }
      }
    };

    let query = r#"
      MATCH (:__Organization)
      RETURN count(*)
    "#;
    let result = graph.send(name, query, None);
    info!("The graph replied with:\n{:?}", result);

    Ok(FhlGraph {
      name: name.to_string(),
      url: url.to_string(),
      db: Arc::new(graph),
    })
  }

  fn bootstrap(&mut self) -> Result<()> {
    let mut orgs: HashMap<Uuid, Organization> = HashMap::new();

    // Temporary data for testing. This should be in a Grapht::QuerySet
    for i in 1..9 {
      let mut base = T::new(
        &format!("pretty_{}", i)[..],
        &format!("full org name: {}", i)[..],
        dec!(0),
      );

      base.set_parent(root.clone());
      let arc_base = Arc::new(Mutex::new(base));
      let _ = nodes.insert(arc_base.lock().unwrap().guid.clone(), arc_base.clone());

      for j in 1..9 {
        let mut child = T::new(
          &format!("pretty_{}_{}", i, j)[..],
          &format!("{}, Child of {} ", j, i)[..],
          Decimal::from(i * j),
        );
        child.set_parent(arc_base.clone());
      }
    }

    todo!("Still working on the bootstrap")
  }

  /// Get a pointer to the primary graph
  pub fn graph(&self) -> Arc<Grapht> {
    self.db.clone()
  }
}
