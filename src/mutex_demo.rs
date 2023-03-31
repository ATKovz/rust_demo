use std::{
  sync::{mpsc, Mutex},
  thread,
  time::Duration,
};

pub fn main() {
  let m = Mutex::new("lock by mutex");

  let (sender, _rx) = mpsc::channel();
  let sender_2 = mpsc::Sender::clone(&sender);

  let _handle_1 = thread::spawn(move || {
      sender.send("!!!").unwrap();
  });

  let handle_2 = thread::spawn(move || {
      for _i in 0..10 {
          let mut s = String::from("???") + "aa";
          let s2 = s + "_tail";
          let s3 = &s2[..];
          sender_2.send("s2").unwrap(); // pass s2 instead of s2[..]
          thread::sleep(Duration::new(1, 1));
      }
  });

  handle_2.join().unwrap();
}
