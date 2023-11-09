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
use swc_ecmascript::parser::{EsConfig, TsConfig};

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

  pub fn parse(
    &self,
    file: &str,
    should_recursion: bool,
    should_resolve: bool,
  ) -> HashMap<Arc<String>, ImportNode> {
    let mut visitor = ImportVisitor::new(ImportResolver::new(should_resolve));

    GLOBALS.set(&Globals::new(), || {
      let mut visited_files: Vec<Arc<String>> = vec![];
      self.recursion_parse(file, &mut visitor, &mut visited_files, should_recursion);

      visitor.import_node.map
    })
  }

  fn recursion_parse<'a>(
    &self,
    file: &str,
    visitor: &mut ImportVisitor,
    visited_files: &mut Vec<Arc<String>>,
    should_recursion: bool,
  ) {
    let resolved_file = &visitor.resolver.resolve_file(&self.root, file);
    let process_id = if visitor.resolver.should_resolve {
      resolved_file
    } else {
      file
    };
    visitor.set_process_id(process_id);
    visitor.create_node(process_id);
    visited_files.push(resolved_file.clone());

    self.parse_file(resolved_file, visitor);

    if !should_recursion {
      return;
    }

    // https://docs.rs/im/latest/im/hashmap/struct.HashMap.html#impl-Clone
    // Hashmap.clone is a shallow clone, so it won't impact performance
    let map = visitor.import_node.map.clone();

    for (id, node) in map {
      if visited_files.contains(&id) || node.kind != ImportNodeKind::Local {
        continue;
      }
      self.recursion_parse(&id, visitor, visited_files, true);
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
  }

  fn get_options(&self, file: &str) -> (Syntax, bool) {
    if file.ends_with(".ts") {
      return (Syntax::Typescript(Default::default()), true);
    }

    if file.ends_with(".tsx") {
      return (
        Syntax::Typescript(TsConfig {
          tsx: true,
          ..Default::default()
        }),
        true,
      );
    }

    if file.ends_with(".jsx") {
      return (
        Syntax::Es(EsConfig {
          jsx: true,
          ..Default::default()
        }),
        false,
      );
    }

    return (Syntax::default(), false);
  }
}
