use std::env;

pub fn main () {
  for i in env::args() {
    
    println!("{:?}", i)
  }
}