use std::sync::Arc;

use swc_ecmascript::{
  ast::{ImportDecl, ImportSpecifier},
  visit::{noop_visit_type, Visit},
};

use crate::{
  node::{self, ImportLink, ImportLinkKind, ImportNode, ImportNodeKind, ImportNodeMap},
  resolver::ImportResolver,
};

pub(crate) struct ImportVisitor {
  pub(crate) import_node: ImportNodeMap,
  process_id: Option<Arc<String>>,
  pub(crate) resolver: ImportResolver,
}

impl ImportVisitor {
  pub(crate) fn new(resolver: ImportResolver) -> Self {
    Self {
      import_node: ImportNodeMap::new(),
      process_id: None,
      resolver,
    }
  }

  pub(crate) fn set_process_id(&mut self, id: Arc<String>) {
    self.process_id = Some(id.clone());
  }

  /// insert node if not exist
  pub(crate) fn create_node(&mut self, id: Arc<String>) {
    if self.import_node.map.get_mut(&id).is_none() {
      self.import_node.create_node(&id);
    }
  }

  fn resolve_from_process_id(&self, request: &str) -> ImportNode {
    let id = self
      .resolver
      .resolve_module(self.process_id.as_ref().unwrap(), request);

    ImportNode {
        kind: ImportNodeKind::compute(&id, &request),
        id: Arc::new(id),
        ..ImportNode::default()
    }
  }

  fn insert_process_node_depent(&mut self, module: ImportNode) -> &mut ImportNode {
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
      return;
    }

    let module_node =
      self.resolve_from_process_id(&String::from_utf8_lossy(&import.src.value.as_bytes()));
    let module_id = module_node.id.clone();
    let process_node = self.insert_process_node_depent(module_node);

    let imports = process_node.import.as_mut().unwrap();
    let mut ident: Vec<node::ImportSpecifier> = vec![];
    for spec in import.specifiers.iter() {
      match spec {
        ImportSpecifier::Named(ref named_spec) => {
          if named_spec.is_type_only {
            continue;
          }

          let name = named_spec.local.sym.to_string();
          ident.push(node::ImportSpecifier {
            name: name.clone(),
            _as: name,
          });
        }
        ImportSpecifier::Default(ref default_spec) => {
          let _as = default_spec.local.sym.to_string();
          ident.push(node::ImportSpecifier {
            name: "default".into(),
            _as,
          })
        }
        ImportSpecifier::Namespace(ref namespace) => ident.push(node::ImportSpecifier {
          name: "*".into(),
          _as: namespace.local.sym.to_string(),
        }),
      }
    }

    imports.push(ImportLink {
      id: module_id,
      kind: ImportLinkKind::Static,
      ident,
    });

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
