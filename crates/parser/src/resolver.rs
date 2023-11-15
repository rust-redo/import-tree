use std::{path::Path, sync::Arc};

use oxc_resolver::{ResolveError, ResolveOptions, Resolver};

use crate::node::{ImportNode, ImportNodeKind};

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

pub struct ImportResolver {
  resolver: Resolver,
  pub(crate) should_resolve: bool,
}

impl ImportResolver {
  pub fn new(should_resolve: bool) -> Self {
    Self {
      should_resolve,
      resolver: Resolver::new(ResolveOptions {
        builtin_modules: true,
        extensions: vec![".js".to_string(), ".ts".to_string()],
        ..ResolveOptions::default()
      }),
    }
  }

  pub(crate) fn resolve_file(&self, root: &str, file: &str) -> Arc<String> {
    let file_path = Path::new(file);

    Arc::new(if file_path.is_absolute() {
      file.to_owned()
    } else {
      Path::new(root)
        .join(file_path)
        .to_str()
        .unwrap()
        .to_string()
    })
  }

  pub(crate) fn resolve_module(&self, root: &str, request: &str) -> ImportNode {
    let root_path = Path::new(root).parent().unwrap_or_else(|| Path::new("/"));
    let id = if self.should_resolve {
      match self.resolver.resolve(root_path, request) {
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

    ImportNode {
      kind: self.get_node_kind(&id, &request),
      id: Arc::new(id),
      ..ImportNode::default()
    }
  }

  fn get_node_kind(&self, id: &str, request: &str) -> ImportNodeKind {
    if BUILTINS.contains(&id) || BUILTINS.contains(&id.replace("node:", "").as_str()) {
      return ImportNodeKind::Builtin;
    }

    if id.contains("/node_modules/") || (!id.starts_with('.') && !request.starts_with('.')) {
      return ImportNodeKind::NodeModules;
    }

    ImportNodeKind::Local
  }
}
