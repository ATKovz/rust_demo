use std::{rc::{Rc, Weak}, cell::RefCell };


#[derive(Debug)]
struct L (i32);

fn rc () {
  let val = L(39_999);
  let rc_val = Rc::new(val);
  // let b = Box::new(val);
  // // 所有权丢失
  // let b1 = Box::new(val);

  let _rc_1 = Rc::clone(&rc_val);
  // 所有权正常, 每次clone会让 RC + 1，释放-1，为0时释放
  // let rc_2 = Rc::clone(&rc_val);
  // let rc_3 = rc_val.clone();
  let rc_4 = rc_val.clone();


  // ref_cell 是运行时检查，只允许单一所有者可变/不可变借用
  // rc是编译时检查，允许多个不可变借用，主要用于闭包调用之类的场景
  // Box是编译时单一可变/不可变
  // ref_cell 具有内部可变，也就是能实现一个immut变量对外不可变，内部可变（mut）
  // 也就是跨闭包或者函数借用 只能用 RC 确保引用计数来保证生命周期，ref_cell/box 都单一借用
  // 而要可变地借用不可变变量就必须是 ref_cell，然后用borrow_mut 在运行时检查，不会compile出错，只会panic
  // 结合使用就可以有多个借用的同时可变，参考rc_with_ref_cell 
  let ref_cell = RefCell::new("abc");

  let a = String::from("test_mutable_in_imutable");

  let ref_a = RefCell::new(a);

  let mut mut_borrow = ref_a.borrow_mut();
  mut_borrow.push_str("_mutted!!");
  // borrow第二次时会panic，但是编译不会出错，因为这和上面违背了，不止一个immut变量 
  // let mut mut_borrow_a = ref_a.borrow_mut();
  // mut_borrow_a.push_str("_mutted!!");
  println!("change immut val with refcell {:?}", ref_a);

  println!("rc val {:?} {}", rc_4, Rc::strong_count(&rc_val))
}

fn rc_with_ref_cell () {
  // 结合后就可以创建多个可变引用了
  let str = String::from("string");

  let ref_cell = RefCell::new(str);
  let rc = Rc::new(ref_cell);

  let b = rc.clone();
  let c = rc.clone();
  b.borrow_mut().push_str("_b");
  c.borrow_mut().push_str("_c");
  println!("{:?} {:?}", b, c)
}
#[derive(Debug)]
struct Loop {
  next: Option<Rc<RefCell<Loop>>>,
}


impl Loop {
  fn set_next (&mut self, next: Option<Rc<RefCell<Loop>>>) {
    self.next = next;
  }
}


fn overflow () {
  let mut a = Loop{ next: None };
  let a_ref = RefCell::new(a);
  let a_rc = Rc::new(a_ref);
  let b = Loop{ next: Some(a_rc.clone()) };

  let b_rc_mut = Some(Rc::new(RefCell::new(b)).clone());
  a_rc.clone().borrow_mut().set_next(b_rc_mut);
  println!("{:?}", a_rc);
  // 使用了强引用所以无限循环
}
#[derive(Debug)]
struct LoopW {
  next: Option<Weak<RefCell<LoopW>>>,
}


impl LoopW {
  fn set_next (&mut self, next: Option<Weak<RefCell<LoopW>>>) {
    self.next = next;
  }
}


fn rc_weak () {

  let a = LoopW{ next: None };
  let a_ref = RefCell::new(a);
  let a_rc = Rc::new(a_ref);
  let a_weak = Rc::downgrade(&a_rc);
  let b = LoopW{ next: Some(a_weak) };

  let b_rc = Rc::new(RefCell::new(b)).clone();
  let b_rc_mut = Some(Rc::downgrade(&b_rc));
  a_rc.borrow_mut().set_next(b_rc_mut);
  println!("{:?}", a_rc);
  // 使用了强引用所以无限循环
}


pub fn main () {
  // rc()

  // rc_with_ref_cell()
  rc_weak()
}