
trait Show {
  fn show (&self) {
    println!("need to override!")
  }
}

trait ShowTwice {
  fn show_twice (&self) {
    println!("SHOW TWICE!")
  }
}

struct Demo {
  name: String
}

impl Demo {
  fn new (name: String) -> Self {
    Demo {
      name: name,
    }
  }
}

impl Show for Demo {
  fn show (&self) {
      println!("overrideed!")
  }
}

impl<T: Show> ShowTwice for T {}

fn call_show (arg: impl Show) {
  arg.show();
}

pub fn main () {
  let sth = Demo::new(String::from("teo"));
  sth.show_twice();
  call_show(sth);
}