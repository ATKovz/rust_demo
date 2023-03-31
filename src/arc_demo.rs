use std::{thread, time::Duration, sync::mpsc::channel};
use std::sync::mpsc;


pub fn main () {
  let (sender, rx) = channel();

  // 不同thread 需要通过 clone 来创建sender
  let sender_2 = mpsc::Sender::clone(&sender);
  let handle = thread::spawn(move || {
    let str_list = [
      "hello",
      "little",
      "pony",
    ];
  for i in str_list {
      // println!("{}", i);
      sender.send(i).unwrap();
      thread::sleep(Duration::new(1, 4));
    }
  });
  let handle2 = thread::spawn(move || {
    let str_list = [
      "Teo",
      "Said",
      "Jerk",
    ];
    for i in str_list {
      // println!("{}", i);
      sender_2.send(i).unwrap();
      thread::sleep(Duration::new(1, 0));
    }
  });

  for n in rx {

    println!("from sender: {}", n);
  }
  // 阻塞住当前线程不让他在handle对应的thread完成之前退出
  // 如果只是发一条的话，多条能持续接受就不用
  // handle.join();

  println!("run!")

}