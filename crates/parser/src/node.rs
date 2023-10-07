use std::{collections::HashMap, sync::Arc};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ImportSpecifier {
  pub name: String,
  pub _as: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ImportLinkKind {
  Static,
  Dynamic,
  Require,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ImportNodeKind {
  #[default]
  Local,
  Builtin,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportLink {
  pub id: Arc<String>,
  #[serde(rename = "type")]
  pub kind: ImportLinkKind,
  pub ident: Vec<ImportSpecifier>,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportNode {
  pub id: Arc<String>,
  #[serde(rename = "type")]
  pub kind: ImportNodeKind,
  pub importer: Option<Vec<Arc<String>>>,
  pub import: Option<Vec<ImportLink>>,
}

#[derive(Serialize, Debug)]
pub struct ImportNodeMap {
  #[serde(flatten)]
  pub(crate) map: HashMap<Arc<String>, ImportNode>,
}

impl ImportNodeMap {
  pub(crate) fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
  }

  pub(crate) fn create_node(&mut self, id: &str) {
    let arc_id = Arc::new(id.to_owned());
    self.map.insert(
      arc_id.clone(),
      ImportNode {
        id: arc_id.clone(),
        ..ImportNode::default()
      },
    );
  }

  pub(crate) fn insert_node_depend(&mut self, id: &str, module: ImportNode) -> &mut ImportNode {
    let root_id = Arc::new(id.to_owned());
    let module_id = module.id.clone();
    let module_node = self.map.get_mut(&module_id);

    let module_node = match module_node {
      Some(m) => m,
      None => {
        self.map.insert(module_id.clone(), module);
        self.map.get_mut(&module_id.clone()).unwrap()
      }
    };

    if module_node.importer.is_none() {
      module_node.importer = Some(vec![]);
    }

    module_node.importer.as_mut().unwrap().push(root_id.clone());

    let root_node = self.map.get_mut(&root_id.clone()).unwrap();

    if root_node.import.is_none() {
      root_node.import = Some(vec![]);
    }

    root_node
  }
}
