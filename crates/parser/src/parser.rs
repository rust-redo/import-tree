use crate::{node::ImportNode, visitor::ImportVisitor};
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

  pub fn parse(&self, file: &str) -> HashMap<Arc<String>, ImportNode> {
    let mut visitor = ImportVisitor::new();
    let full_file = &visitor
      .resolve(env::current_dir().unwrap().to_str().unwrap(), file)
      .id;

    GLOBALS.set(&Globals::new(), || {
      visitor.set_process_id(full_file);
      visitor.create_node(full_file);

      self.parse_file(full_file, &mut visitor);

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
