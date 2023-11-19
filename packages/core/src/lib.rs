#![deny(clippy::all)]

use napi::bindgen_prelude::Buffer;
use parser::{Alias, AliasValue, ImportResolver, Parser as InternalParser};

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Parser {
  parser: InternalParser,
}

#[napi]
impl Parser {
  #[napi(constructor)]
  pub fn new(root: Option<Buffer>, alias: Option<Buffer>) -> Self {
    let root = Self::compute_root(root);
    let alias = Self::compute_alias(&root, alias);
    Self {
      parser: InternalParser::new(root, alias),
    }
  }

  #[napi]
  pub fn parse(&self, files: Buffer, depth: Option<u8>, should_resolve: Option<bool>) -> Buffer {
    let files = String::from_utf8_lossy(&files).to_string();
    let files = files.split(",").collect();

    serde_json::to_string(&self.parser.parse(files, depth, should_resolve))
      .unwrap()
      .as_bytes()
      .into()
  }

  fn compute_root(root: Option<Buffer>) -> Option<String> {
    match root {
      Some(buf) => Some(String::from_utf8_lossy(&buf).to_string()),
      _ => None,
    }
  }

  fn compute_alias(root: &Option<String>, alias: Option<Buffer>) -> Option<Alias> {
    match alias {
      Some(buf) => {
        let alias_str = String::from_utf8_lossy(&buf).to_string();
        let alias: Alias = alias_str
          .split(" ")
          .map(|s| {
            let kv: Vec<&str> = s.split(":").collect();
            let paths: Vec<AliasValue> = kv[1]
              .split(",")
              .map(|p| {
                return AliasValue::Path(ImportResolver::resolve_file(root.as_ref().unwrap(), p));
              })
              .collect();
            return (kv[0].to_owned(), paths);
          })
          .collect();
        Some(alias)
      }
      _ => None,
    }
  }
}
