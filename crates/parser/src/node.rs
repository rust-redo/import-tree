use std::{rc::Rc, cell::RefCell, collections::HashMap};

use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct ImportSpecifier {
  pub name: String,
  pub _as: String
}

#[derive(Serialize, Debug)]
pub enum ImportKind {
  Static,
  Dynamic,
  Require
}

#[derive(Serialize, Debug)]
pub struct ImportLink {
  pub id: String,
  #[serde(rename="type")]
  pub kind: ImportKind,
  pub ident: Vec<ImportSpecifier>,
}

#[derive(Serialize, Debug, Default)]
pub struct ImportNode {
  pub importer: Option<Vec<String>>,
  pub import: Option<Vec<ImportLink>>,
}

#[derive(Serialize, Debug)]
pub struct ImportNodeMap {
  #[serde(flatten)]
  pub(crate) map: HashMap<String, ImportNode>
}

impl ImportNodeMap {
  pub fn new() -> Self {
    Self {
      map: HashMap::new()
    }
  }

  pub fn get_or_insert_node(&mut self, id: &str) -> &mut ImportNode {
    let node = self.map.get_mut(id);

    if node.is_none() {
      self.map.insert(id.to_owned(), ImportNode::default());
    }

    self.map.get_mut(id).unwrap()
  }

  pub fn insert_node(&mut self, id: &str) {
    self.map.insert(id.to_owned(), ImportNode::default());
  }

  pub fn insert_node_depend(&mut self, id: &str, module: &str) -> &mut ImportNode {
    let module_node = self.map.get_mut(module);

    let module_node = match module_node {
      Some(m) => {m},
      None => {
        self.map.insert(module.to_owned(),  ImportNode::default());
        self.map.get_mut(module).unwrap()
      }
    };

    if module_node.importer.is_none() {
      module_node.importer = Some(vec![]);
    }

    module_node.importer.as_mut().unwrap().push(id.to_owned());

    let root_node = self.map.get_mut(id).unwrap();

    if root_node.import.is_none() {
      root_node.import = Some(vec![]);
    }

    root_node
  }
}