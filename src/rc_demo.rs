use std::rc::Rc;


#[derive(Debug)]
struct L (i32);

fn rc () {
  let val = L(39_999);
  let rc_val = Rc::new(val);
  // let b = Box::new(val);
  // // 所有权丢失
  // let b1 = Box::new(val);

  let rc_1 = Rc::clone(&rc_val);
  // 所有权正常, 每次clone会让 RC + 1，释放-1，为0时释放
  // let rc_2 = Rc::clone(&rc_val);
  // let rc_3 = rc_val.clone();
  let rc_4 = rc_val.clone();

  println!("rc val {:?} {}", rc_4, Rc::strong_count(&rc_val))
}

pub fn main () {
  rc()
}