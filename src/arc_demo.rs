use std::{thread, time::Duration, sync::mpsc::channel};
use std::sync::mpsc;

pub fn thread () {
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


pub fn arc_demo() {
  // let m = Mutex::new(String::from("muteX"));
  // MuteX 只能提供锁，线程间安全需要Arc来保证，因为RC无法确保增加/减少 reference count的过程不会被其他插入
  // 而 Atom  rc（ARC）提供了原子化的rc，性能比RC差因为有一定的开销，但是确保了线程间RC安全
  let m = Arc::new(Mutex::new(0));

  // let (sender, _rx) = mpsc::channel();
  let mut handlers = vec![];
  
  for _ in 0..10 {
    // let sender = mpsc::Sender::clone(&sender);
    let mm = Arc::clone(&m);
    let handler = thread::spawn(move || {
      let mut str = mm.lock().unwrap();
      // let marker = format!("from {} ", _i);
      *str += 1;
      // sender.send(str);
    });

    handlers.push(handler);

  }

  println!("run {:?}", handlers);

  for h in handlers {
    // println!("{:?}", h);
    h.join().unwrap();
  }
  println!("{}", *m.lock().unwrap())
/* 
  for i in _rx.recv() {
    println!("{}", i)
  } */

}



pub fn main () {
  arc_demo();
}