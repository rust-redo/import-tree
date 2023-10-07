use crate::{visitor::ImportVisitor, node::{ImportNode, ImportNodeMap}};
use std::{path::Path, sync::Arc, collections::HashMap, rc::Rc};
use swc_core::{
  base::{
    config::{Config, IsModule, JscConfig, ModuleConfig, Options},
    Compiler,
  },
  common::{
    errors::{ColorConfig, Handler},
    Globals, Mark, SourceMap, GLOBALS,
  },
  ecma::{
    ast::{EsVersion, ImportDecl},
    parser::Syntax,
    transforms::base::resolver,
    visit::{noop_visit_type, Visit, VisitMut, VisitMutWith, VisitWith},
  },
  node::{deserialize_json, get_deserialized, MapErr},
};

pub struct Parser {
  source_map: Arc<SourceMap>,
  handler: Handler,
  compiler: Compiler,
}

impl Parser {
  pub fn new() -> Parser {
    let source_map = Arc::<SourceMap>::default();

    Parser {
      source_map: source_map.clone(),
      handler: Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone())),
      compiler: swc::Compiler::new(source_map.clone()),
    }
  }


  /// parse signle file and its dependency
  // pub fn parse_file_with_dependency(&self, file: &str) {
  //   return self.parse_file(file);
  // }

  pub fn parse(&self, file: &str) -> HashMap<String, ImportNode> {
    let mut visitor = ImportVisitor::new();

    GLOBALS.set(&Globals::new(), || {
      visitor.set_process_id(file);
      visitor.insert_node(file);

      self.parse_file(file, &mut visitor);

      visitor.import_node.map
    })
  }

  /// parse single js file
  fn parse_file(&self, file: &str, visitor: &mut ImportVisitor) {
    // GLOBALS.set(&Globals::new(), || {
      // let mut visitor = ImportVisitor::new(file.to_string());
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
