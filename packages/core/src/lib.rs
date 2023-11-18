#![deny(clippy::all)]

use napi::bindgen_prelude::Buffer;
use parser::Parser as InternalParser;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Parser {
  parser: InternalParser,
}

#[napi]
impl Parser {
  #[napi(constructor)]
  pub fn new(root: Option<Buffer>) -> Self {
    Self {
      parser: InternalParser::new(match root {
        Some(buf) => Some(String::from_utf8_lossy(&buf).to_string()),
        _ => None,
      }),
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
}
