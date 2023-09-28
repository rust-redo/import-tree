use parser::{Parser};

#[test]
fn test_es() {
  let parser = Parser::new();

  println!("{}", serde_json::to_string(&parser.parse("./tests/es.js")).unwrap());
  println!("{}", serde_json::to_string(&parser.parse("./tests/ts.ts")).unwrap());
}

#[test]
fn test_ts() {
  let parser = Parser::new();

  println!("{}", serde_json::to_string(&parser.parse("./tests/ts.ts")).unwrap());
}
