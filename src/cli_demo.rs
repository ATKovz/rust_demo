use std::env;
use std::io;
use std::fs;
use std::io::Read;

struct Cfg<'a > {
  filename: &'a str,
  handler: &'a str,
}

impl<'a > Cfg<'a > {
  fn new (filename: &'a str, handler: &'a str) -> Cfg {
    Cfg {
      filename,
      handler
    }
  }
}

fn parse_params (args: &Vec<String>) -> Cfg {
  Cfg::new(
    &args[1].as_str(),
    &args[2].as_str()
  )
}

fn open_file <'a>(path: &str, target: &'a mut String) -> &'a String {
  let mut res = fs::File::open(path).expect("【Error】sth was catch by user");

  res.read_to_string(target);

  target
}

pub fn main () {
  let params = env::args().collect();
  let Cfg = parse_params(&params);
  let mut str_result: String = String::new();

  open_file(&Cfg.filename, & mut str_result);
  println!("{:?}", str_result)
}
