use std::{path::Path, sync::Arc};

use oxc_resolver::{Resolution, ResolveError, ResolveOptions, Resolver};

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
  should_resolve: bool
}

impl ImportResolver {
  pub fn new(should_resolve: bool) -> Self {
    Self {
      should_resolve,
      resolver: Resolver::new(ResolveOptions {
      builtin_modules: true,
      ..ResolveOptions::default()
    }),}
  }

  pub(crate) fn resolve(&self, root: &str, request: &str) -> ImportNode {
    let path = Path::new(root).parent().unwrap_or_else(|| Path::new("/"));
    let id = if self.should_resolve {
      match self.resolver.resolve(path, request) {
        Ok(res) => res.full_path().to_string_lossy().to_string(),
        Err(err) => match err {
          // builtin module
          ResolveError::Builtin(file_name) => file_name,
          _ => "".to_owned(),
        },
      }
    } else {
      request.to_owned()
    };

    ImportNode {
      kind: self.get_node_kind(&id),
      id: Arc::new(id),
      ..ImportNode::default()
    }
  }

  fn get_node_kind(&self, id: &str) -> ImportNodeKind {
    if id.contains("/node_modules/") {
      return ImportNodeKind::NodeModules;
    }

    if BUILTINS.contains(&id) {
      return ImportNodeKind::Builtin;
    }

    ImportNodeKind::Local
  }
}