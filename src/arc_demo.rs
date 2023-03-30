use std::{thread, time::Duration, sync::mpsc::channel};


pub fn main () {
  let (sender, rx) = channel();
  let handle = thread::spawn(move || {
  for i in 1..10 {
    
      println!("{}", i);
      sender.send(i);
      thread::sleep(Duration::new(1, 1));
    }
  });
/*   for i in 1..10 {
    // 2个线程会交替执行
    println!("main {}", i);
    thread::sleep(Duration::new(1, 1));
  } */
  for n in rx {

    println!("from sender{}", n);
  }
  // 阻塞住当前线程不让他在handle对应的thread完成之前退出
  handle.join();

}