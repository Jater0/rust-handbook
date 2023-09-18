use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn hello_from_ferris() {
  let stdout = stdout();
  let message = String::from("Hello, I am ferris");
  let width = message.chars().count();

  let mut writer = BufWriter::new(stdout.lock());
  say(&message, width, &mut writer).unwrap();
}