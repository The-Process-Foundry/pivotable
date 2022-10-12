//! An individual table cell

use crate::local::*;
use serde::{Deserialize, Serialize};

pub use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct CellConfig {
  /// An identity to be able to reuse the same layout
  name: Option<String>,

  value: CellValue,

  format: Option<CellFormat>,
}

impl CellConfig {
  pub fn new() -> CellConfig {
    CellConfig::default()
  }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CellBuilder {
  config: CellConfig,
}

impl CellBuilder {
  pub fn new() -> CellBuilder {
    CellBuilder {
      config: CellConfig::default(),
    }
  }
  pub fn set_config(&mut self, config: &CellConfig) -> Result<()> {
    self.config = config.clone();

    Ok(())
  }
}

impl From<CellConfig> for CellBuilder {
  fn from(value: CellConfig) -> Self {
    let mut builder = CellBuilder::new();
    builder.set_config(&value).expect(&format!(
      "Could not create a builder from config: {:?}",
      value
    ));
    builder
  }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CellValue {
  /// A constant value, usually used for headers
  Const(String),

  Query(String),

  Map(HashMap<String, Box<CellValue>>),
}

impl Default for CellValue {
  fn default() -> CellValue {
    CellValue::Query("{{node}}".to_string())
  }
}

#[derive(Clone, Debug, Default, PartialEq, Properties, Serialize, Deserialize)]
pub struct CellFormat {
  template: String,
  class_map: ClassMap,
}

impl CellFormat {
  pub fn new(template: &str) -> CellFormat {
    CellFormat {
      template: template.to_string(),
      class_map: ClassMap::default(),
    }
  }
}
