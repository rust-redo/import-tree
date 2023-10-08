#![deny(clippy::all)]

use parser::Parser as InternalParser;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

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
  pub fn parse(&self, file: String, should_resolve: bool) {
    println!("{}", serde_json::to_string(&self.parser.parse(&file, should_resolve)).unwrap());
  }
}
