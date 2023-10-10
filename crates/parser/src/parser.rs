use crate::{
  node::{ImportNode, ImportNodeKind},
  resolver::ImportResolver,
  visitor::ImportVisitor,
};
use std::{collections::HashMap, env, path::Path, sync::Arc};
use swc_core::{
  base::{config::IsModule, Compiler},
  common::{
    errors::{ColorConfig, Handler},
    Globals, Mark, SourceMap, GLOBALS,
  },
  ecma::{
    ast::EsVersion,
    parser::Syntax,
    transforms::base::resolver,
    visit::{VisitMutWith, VisitWith},
  },
};

pub struct Parser {
  source_map: Arc<SourceMap>,
  handler: Handler,
  compiler: Compiler,
  root: String,
}

impl Parser {
  pub fn new(root: Option<String>) -> Parser {
    let source_map = Arc::<SourceMap>::default();

    Parser {
      source_map: source_map.clone(),
      handler: Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone())),
      compiler: swc::Compiler::new(source_map.clone()),
      root: match root {
        Some(r) => r,
        _ => env::current_dir().unwrap().to_string_lossy().to_string(),
      },
    }
  }

  /// parse signle file and its dependency
  // pub fn parse_file_with_dependency(&self, file: &str) {
  //   return self.parse_file(file);
  // }

  pub fn parse(&self, file: &str, should_resolve: bool) -> HashMap<Arc<String>, ImportNode> {
    let mut visitor = ImportVisitor::new(ImportResolver::new(should_resolve));
    let resolved_file = &visitor.resolver.resolve(&self.root, file).id;

    GLOBALS.set(&Globals::new(), || {
      let mut visited_files: Vec<Arc<String>> = vec![];
      self.recursion_parse(resolved_file, &mut visitor, &mut visited_files);

      visitor.import_node.map
    })
  }

  fn recursion_parse<'a>(
    &self,
    resolved_file: &Arc<String>,
    visitor: &mut ImportVisitor,
    visited_files: &mut Vec<Arc<String>>,
  ) {
    visitor.set_process_id(resolved_file);
    visitor.create_node(resolved_file);
    visited_files.push(resolved_file.clone());

    self.parse_file(resolved_file, visitor);

    // https://docs.rs/im/latest/im/hashmap/struct.HashMap.html#impl-Clone
    // Hashmap.clone is a shallow clone, so it won't impact performance
    let map = visitor.import_node.map.clone();

    for (id, node) in map {
      if visited_files.contains(&id) || node.kind != ImportNodeKind::Local {
        continue;
      }
      self.recursion_parse(&id, visitor, visited_files);
    }
  }

  /// parse single js file
  fn parse_file(&self, file: &str, visitor: &mut ImportVisitor) {
    let fm = self
      .source_map
      .load_file(Path::new(file))
      .expect(&format!("failed to load {}", file));

    let (syntax, is_ts) = self.get_options(file);

    let mut program = self
      .compiler
      .parse_js(
        fm,
        &self.handler,
        EsVersion::latest(),
        syntax,
        IsModule::Unknown,
        None,
      )
      .unwrap();

    program.visit_mut_with(&mut resolver(Mark::new(), Mark::new(), is_ts));

    let module = program.expect_module();
    module.visit_with(visitor);
    // })
  }

  fn get_options(&self, file: &str) -> (Syntax, bool) {
    if file.ends_with(".ts") {
      return (Syntax::Typescript(Default::default()), true);
    }

    return (Syntax::default(), false);
  }
}
