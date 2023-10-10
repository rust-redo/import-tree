#![deny(clippy::all)]

use napi::bindgen_prelude::Buffer;
use parser::Parser as InternalParser;

#[macro_use]
extern crate napi_derive;

#[napi]
pub struct Parser {
  parser: InternalParser
}

#[napi]
impl Parser {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self { parser: InternalParser::new() }
  }

  #[napi]
  pub fn parse(&self, file: Buffer, should_resolve: bool) -> Buffer {
    let file = String::from_utf8_lossy(&file).to_string();
    serde_json::to_string(&self.parser.parse(&file, should_resolve)).unwrap().as_bytes().into()
  }
}
