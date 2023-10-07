use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ImportSpecifier {
  pub name: String,
  pub _as: String,
}

#[derive(Serialize, Debug, Clone)]
pub enum ImportLinkKind {
  Static,
  Dynamic,
  Require,
}

#[derive(Serialize, Debug, Default, Clone)]
pub enum ImportNodeKind {
  #[default]
  Local,
  Builtin,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportLink {
  pub id: String,
  #[serde(rename = "type")]
  pub kind: ImportLinkKind,
  pub ident: Vec<ImportSpecifier>,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportNode {
  pub id: String,
  #[serde(rename = "type")]
  pub kind: ImportNodeKind,
  pub importer: Option<Vec<String>>,
  pub import: Option<Vec<ImportLink>>,
}

#[derive(Serialize, Debug)]
pub struct ImportNodeMap {
  #[serde(flatten)]
  pub(crate) map: HashMap<String, ImportNode>,
}

impl ImportNodeMap {
  pub(crate) fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
  }

  pub(crate) fn create_node(&mut self, id: &str) {
    self.map.insert(
      id.to_owned(),
      ImportNode {
        id: id.to_owned(),
        ..ImportNode::default()
      },
    );
  }

  // pub(crate) fn update_node(&mut self, id: &str, )

  pub(crate) fn insert_node_depend(&mut self, id: &str, module: ImportNode) -> &mut ImportNode {
    let module_id = module.id.to_owned();
    let module_node = self.map.get_mut(&module_id);

    let module_node = match module_node {
      Some(m) => m,
      None => {
        self.map.insert(module_id.to_owned(), module);
        self.map.get_mut(&module_id).unwrap()
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
