mod commands;
mod interpreter;
mod ast;

use std::{str, io::Cursor};

pub fn execute(input: String) -> String {
  let mut writer = Cursor::new(vec!());
  let program = ast::parse(format!("{}\n", input));
  interpreter::run(program, &mut writer);
  let output = writer.get_ref();
  let str_out = str::from_utf8(&output).unwrap();
  str_out.to_owned()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn it_executes_vl_programs(){
    let program = "ihello, world!;w".to_owned();
    let expected_result = "hello, world!";
    let actual_result = execute(program);
    assert_eq!(expected_result, &actual_result);
  }
}
