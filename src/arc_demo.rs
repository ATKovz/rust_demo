use std::{thread, time::Duration};

pub fn main () {
  for i in 1..10 {
    
    thread::spawn(|| {
      println!("{}", i)
    });
  }
}