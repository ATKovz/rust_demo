
struct Memorize<T> {
  memo: Option<usize>,
  func: T
}

impl<T> Memorize<T>  where T: Fn(&str) -> usize {
  fn new (func: T) -> Self {
    Memorize {
      func,
      memo: None,
    }
  }
  fn get_val (&mut self, arg: &str) -> usize {
    if let Some(v) = self.memo {
      return v
    }
    let v = (self.func)(arg);
    self.memo = Some(v);
    v
  }
}


pub fn main () {
  let outer_val = "prefix";
  let expensive_fn = |str: &str| {
    println!("{} keyword, {}, length {}", outer_val, str, str.len());
    str.len()
  };

  let mut mem = Memorize{ func: expensive_fn, memo: None };
  println!("{}", mem.get_val("ddddddd"));
  println!("{}", mem.get_val("aaaaa"));
}