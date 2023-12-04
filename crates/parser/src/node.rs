use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use serde::Serialize;

pub const BUILTINS: &[&str] = &[
  "_http_agent",
  "_http_client",
  "_http_common",
  "_http_incoming",
  "_http_outgoing",
  "_http_server",
  "_stream_duplex",
  "_stream_passthrough",
  "_stream_readable",
  "_stream_transform",
  "_stream_wrap",
  "_stream_writable",
  "_tls_common",
  "_tls_wrap",
  "assert",
  "assert/strict",
  "async_hooks",
  "buffer",
  "child_process",
  "cluster",
  "console",
  "constants",
  "crypto",
  "dgram",
  "diagnostics_channel",
  "dns",
  "dns/promises",
  "domain",
  "events",
  "fs",
  "fs/promises",
  "http",
  "http2",
  "https",
  "inspector",
  "module",
  "net",
  "os",
  "path",
  "path/posix",
  "path/win32",
  "perf_hooks",
  "process",
  "punycode",
  "querystring",
  "readline",
  "repl",
  "stream",
  "stream/consumers",
  "stream/promises",
  "stream/web",
  "string_decoder",
  "sys",
  "timers",
  "timers/promises",
  "tls",
  "trace_events",
  "tty",
  "url",
  "util",
  "util/types",
  "v8",
  "vm",
  "worker_threads",
  "zlib",
];

#[derive(Serialize, Debug, Clone)]
pub struct ImportSpecifier {
  pub name: String,
  #[serde(rename = "as")]
  pub _as: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ImportLinkKind {
  Static,
  Dynamic,
  Require,
}

#[derive(Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub enum ImportNodeKind {
  #[default]
  #[serde(rename = "local")]
  Local,
  #[serde(rename = "builtin")]
  Builtin,
  #[serde(rename = "node_modules")]
  NodeModules,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportLink {
  pub id: Arc<String>,
  #[serde(rename = "type")]
  pub kind: ImportLinkKind,
  pub ident: Vec<ImportSpecifier>,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportNode {
  pub id: Arc<String>,
  #[serde(rename = "type")]
  pub kind: ImportNodeKind,
  pub importer: Option<HashSet<Arc<String>>>,
  pub import: Option<Vec<ImportLink>>,
  #[serde(skip_serializing)]
  pub import_paths: Vec<Vec<Arc<String>>>,
}

impl ImportNodeKind {
  pub(crate) fn compute(id: &str, in_root: bool) -> ImportNodeKind {
    if BUILTINS.contains(&id) || BUILTINS.contains(&id.replace("node:", "").as_str()) {
      return ImportNodeKind::Builtin;
    }

    if id.contains("node_modules/") || !in_root {
      return ImportNodeKind::NodeModules;
    }

    ImportNodeKind::Local
  }
}

#[derive(Serialize, Debug)]
pub struct ImportNodeMap {
  #[serde(flatten)]
  pub(crate) map: HashMap<Arc<String>, ImportNode>,
}

impl ImportNodeMap {
  pub(crate) fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
  }

  pub(crate) fn create_node(&mut self, id: &str) {
    let arc_id = Arc::new(id.to_owned());
    self.map.insert(
      arc_id.clone(),
      ImportNode {
        id: arc_id.clone(),
        import_paths: vec![vec![arc_id]],
        ..ImportNode::default()
      },
    );
  }

  pub(crate) fn insert_node_depend(&mut self, id: &str, module: ImportNode) -> &mut ImportNode {
    unsafe {
      let root_id = Arc::new(id.to_owned());
      let module_id = module.id.clone();
      let module_node = self.map.get_mut(&module_id);

      let module_node = match module_node {
        Some(m) => m,
        None => {
          self.map.insert(module_id.clone(), module);
          self.map.get_mut(&module_id.clone()).unwrap() as *mut ImportNode
        }
      };

      if (*module_node).importer.is_none() {
        (*module_node).importer = Some(HashSet::new());
      }

      (*module_node)
        .importer
        .as_mut()
        .unwrap()
        .insert(root_id.clone());

      let root_node = self.map.get_mut(&root_id.clone()).unwrap() as *mut ImportNode;

      if (*root_node).import.is_none() {
        (*root_node).import = Some(vec![]);
      }

      // add new paths to internal module
      if (*module_node).kind == ImportNodeKind::Local {
        (*module_node)
          .import_paths
          .extend((*root_node).import_paths.iter().map(|paths| {
            let mut new_paths = Vec::from_iter(paths.iter().map(|p| p.clone()));
            new_paths.push(module_id.clone());
            // println!("{:?}", new_paths);
            return new_paths;
          }));

          for paths in (*module_node).import_paths.iter(){
            let mut visited_map:HashMap<Arc<String>, usize> = HashMap::new();
            for (index, p) in paths.iter().enumerate() {
              if visited_map.contains_key(&p.clone()) {
                let start = *visited_map.get(&p.clone()).unwrap();
                // println!("circular {:?}", &paths[start..=index]);
              }

              visited_map.insert(p.clone(), index);
            }
          }
      }

      &mut *root_node
    }
  }
}
