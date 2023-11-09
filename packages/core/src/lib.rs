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
  pub fn parse(&self, file: Buffer, should_recursion: bool, should_resolve: bool) -> Buffer {
    let file = String::from_utf8_lossy(&file).to_string();
    serde_json::to_string(&self.parser.parse(&file, should_recursion, should_resolve))
      .unwrap()
      .as_bytes()
      .into()
  }
}
