use oxc_resolver::{ResolveError, ResolveOptions, Resolver};
use std::path::Path;
use swc_ecmascript::{
  ast::{ImportDecl, ImportSpecifier},
  visit::{noop_visit_type, Visit},
};

use crate::node::{self, ImportLink, ImportLinkKind, ImportNode, ImportNodeKind, ImportNodeMap};

pub(crate) struct ImportVisitor {
  pub(crate) import_node: ImportNodeMap,
  process_id: Option<String>,
  resolver: Resolver,
}

impl ImportVisitor {
  pub(crate) fn new() -> Self {
    Self {
      import_node: ImportNodeMap::new(),
      process_id: None,
      resolver: Resolver::new(ResolveOptions {
        builtin_modules: true,
        ..ResolveOptions::default()
      }),
    }
  }

  pub(crate) fn set_process_id(&mut self, id: &str) {
    self.process_id = Some(id.to_owned());
  }

  pub(crate) fn create_node(&mut self, id: &str) {
    self.import_node.create_node(id);
  }

  pub(crate) fn resolve(&self, root: &str, request: &str) -> ImportNode {
    let path = Path::new(root).parent().unwrap_or_else(|| Path::new("/"));
    let (id, kind) = match self.resolver.resolve(path, request) {
      Ok(res) => (
        res.full_path().to_string_lossy().to_string(),
        ImportNodeKind::Local,
      ),
      Err(err) => match err {
        ResolveError::Builtin(file_name) => (file_name, ImportNodeKind::Builtin),
        _ => ("".to_owned(), ImportNodeKind::Local),
      },
    };

    ImportNode {
      id,
      kind,
      ..ImportNode::default()
    }
  }

  fn resolve_from_process_id(&self, request: &str) -> ImportNode {
    self.resolve(self.process_id.as_ref().unwrap(), request)
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
    let module_id = module_node.id.to_owned();
    let process_node = self.insert_process_node_depent(module_node);

    let imports = process_node.import.as_mut().unwrap();
    let mut ident: Vec<node::ImportSpecifier> = vec![];

    for spec in import.specifiers.iter() {
      match spec {
        ImportSpecifier::Named(ref named_spec) => {
          let name = named_spec.local.sym.to_string();
          ident.push(node::ImportSpecifier {
            name: name.clone(),
            _as: name,
          });
        }
        _ => {}
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
