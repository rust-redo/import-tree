use std::{collections::HashMap, rc::Rc, cell::RefCell, os::unix::process};

use swc_ecmascript::{
  ast::{ImportDecl, ImportSpecifier},
  visit::{noop_visit_type, Visit},
};

use crate::node::{self, ImportKind, ImportNode, ImportLink, ImportNodeMap};

pub(crate) struct ImportVisitor {
  pub(crate) import_node: ImportNodeMap,
  process_id: Option<String>
}

impl ImportVisitor {
  pub(crate) fn new() -> Self {
    Self {
      import_node: ImportNodeMap::new(),
      process_id: None
    }
  }

  pub(crate) fn set_process_id(&mut self, id: &str) {
    self.process_id = Some(id.to_owned());
  }

  pub(crate) fn insert_node(&mut self, id: &str) {
    self.import_node.insert_node(id);
  }

  fn insert_process_node_depent(&mut self,  module: &str) -> &mut ImportNode {
    let process_id = self.process_id.clone().unwrap();
    self.import_node.insert_node_depend(&process_id, module)
  }
}

impl Visit for ImportVisitor {
  noop_visit_type!();

  // fn visit_mut_import_named_specifier(&mut self, import: &mut ImportNamedSpecifier) {
  //     dbg!(&import.imported);
  //     dbg!(&import.local);
  //     dbg!(&import.is_type_only);
  // }

  fn visit_import_decl(&mut self, import: &ImportDecl) {
    if import.type_only {
      return
    }

    let module = String::from_utf8_lossy(&import.src.value.as_bytes()).to_string();
    let process_node = self.insert_process_node_depent(&module);

    let imports = process_node.import.as_mut().unwrap();
    let mut ident:Vec<node::ImportSpecifier> = vec![];

    for spec in import.specifiers.iter() {
      match spec {
        ImportSpecifier::Named(ref named_spec) => {
          let name = named_spec.local.sym.to_string();
          ident.push(node::ImportSpecifier {
            name: name.clone(),
            _as: name,
          });
        },
        _ => {}
      }
    }

    imports.push(ImportLink {id: module, kind: ImportKind::Static, ident});

    // println!("serde {}", serde_json::to_string(&self.import_node.import).unwrap());
    // dbg!(&import.specifiers);
  }

  // fn visit_mut_ts_import_equals_decl(&mut self, import: &mut TsImportEqualsDecl) {
  //   dbg!(&import.id);
  //   dbg!(&import.module_ref);
  //   dbg!(&import.is_export);
  //   dbg!(&import.is_type_only);
  //   dbg!(&import.span);
  // }
}
