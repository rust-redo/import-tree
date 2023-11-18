use std::{path::Path, sync::Arc};

use oxc_resolver::{ResolveError, ResolveOptions, Resolver};

use crate::node::{ImportNode, ImportNodeKind};

pub struct ImportResolver {
  resolver: Resolver,
  root: Arc<String>,
  pub(crate) should_resolve: bool,
}

impl ImportResolver {
  pub fn new(root: Arc<String>, should_resolve: bool) -> Self {
    Self {
      root,
      should_resolve,
      resolver: Resolver::new(ResolveOptions {
        builtin_modules: true,
        extensions: vec![".js".to_string(), ".ts".to_string(),".jsx".to_string(), ".tsx".to_string()],
        ..ResolveOptions::default()
      }),
    }
  }

  /// return file absolute path based on source
  pub(crate) fn resolve_file(&self, source: &str, file: &str) -> String {
      Path::new(source)
        .join(Path::new(file))
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
  }

  pub(crate) fn resolve_relative_root(&self, file: &str) -> String {
    if file.starts_with(self.root.as_ref()) {
      let mut root_str = self.root.as_ref().to_string();
      let slash = "/";
      if !root_str.ends_with(slash) {
        root_str.push_str(slash)
      }
      
      return file.replace(&root_str, "");
    }

      return file.replace("./", "");
  }

    /// return module absolute path based on source
  pub(crate) fn resolve_module(&self, source: &str, request: &str) -> String {
    let source_dir = &self.resolve_file(&self.root, source);
    let source_dir = Path::new(source_dir).parent().unwrap_or_else(|| Path::new("/"));
    let id = if self.should_resolve {
      match self.resolver.resolve(source_dir, request) {
        Ok(res) => res.full_path().to_string_lossy().to_string(),
        Err(err) => match err {
          // builtin module
          ResolveError::Builtin(file_name) => file_name,
          _ => request.to_owned(),
        },
      }
    } else {
      request.to_owned()
    };

    self.resolve_relative_root(&id)
  }
}
