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
  root: Arc<String>,
}

impl Parser {
  pub fn new(root: Option<String>) -> Parser {
    let source_map = Arc::<SourceMap>::default();

    Parser {
      source_map: source_map.clone(),
      handler: Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(source_map.clone())),
      compiler: swc::Compiler::new(source_map.clone()),
      root: Arc::new(match root {
        Some(r) => r,
        _ => env::current_dir().unwrap().to_string_lossy().to_string(),
      }),
    }
  }

  pub fn parse(
    &self,
    files: Vec<&str>,
    depth: Option<u8>,
    should_resolve: Option<bool>,
  ) -> HashMap<Arc<String>, ImportNode> {
    let wrapped_depth = depth.unwrap_or(2);
    let wrapped_should_resolve = should_resolve.unwrap_or(true);
    let mut visitor = ImportVisitor::new(ImportResolver::new(self.root.clone(), wrapped_should_resolve));

    GLOBALS.set(&Globals::new(), || {
      for file in files.iter() {
        self.deep_parse(file, &mut visitor, if wrapped_should_resolve {wrapped_depth} else { 1 });
      }

      visitor.import_node.map
    })
  }

  fn deep_parse<'a>(
    &self,
    file: &str,
    visitor: &mut ImportVisitor,
    mut depth: u8,
  ) {
    let mut file_queue = vec![Arc::new(file.to_owned())];
    let mut processed_ids: HashMap<Arc<String>, bool> = HashMap::new();
    let mut current_count = 1;
    let mut next_count = 0;

      while file_queue.is_empty() == false && depth > 0 {
        let target_file = file_queue.pop().unwrap();
        let resolved_file = Arc::new(visitor.resolver.resolve_file(&self.root, &target_file));
        let process_id = Arc::new(visitor.resolver.resolve_relative_root(&target_file));
    
        if processed_ids.contains_key(&process_id.clone()) == false {
          processed_ids.insert(process_id.clone(), true);

          visitor.set_process_id(process_id.clone());
          visitor.create_node(process_id.clone());
          self.parse_file(&resolved_file, visitor);

          let map = visitor.import_node.map.clone();

          for (id, node) in map {
            if processed_ids.contains_key(&id) || node.kind != ImportNodeKind::Local {
                continue;
            }
            next_count += 1;
            file_queue.push(id);
          }
      }

        current_count -= 1;
        
        if current_count == 0 {
          current_count = next_count;
          depth -= 1;
        }
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
