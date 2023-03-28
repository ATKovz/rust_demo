use std::ops::Deref;

enum List {
  // 直接使用List编译会报错，因为编译器会无限递归进去，而Box是一块不确定大小指向List动态指针，不会递归下去
  // box 只提供间接访问内存和分配堆的能力，其实平时很多东西已经内置了Box操作，所以才需要解引用获取值
  Cons(i32, Box<List>),
  Nil,
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    println!("deref");
    &self.0
  }
}

impl<T> MyBox<T> {
  fn new (v: T) -> MyBox<T> {
    MyBox(v)
  }
}

fn plt (n: &str) {
  println!("{}", n);
}

pub fn boxdemo () {
  // val这块内存
  let mut val = 12;
  // 本质上和指针差不多，但是能开到堆里
  let num = Box::new(val);
  // 指向val这块内存的指针

  let mut ref_val = &val;
  let mut  deref_val = *ref_val;
  // 已经没法变了，因为同一块内存只能被一个变量访问，这里被 deref_val 拿了
  // 所有权系统规定值只能被一个变量拥有，而指针可以被多个拥有
  // val = 123;
  let c = *num + 3;
  deref_val = 123;
  println!("{} {} {}", val, ref_val, deref_val);
}

pub fn main () {
  let b = MyBox::new("aaa");
  let test_deref = Box::new("abc");

  let c=  String::from("abc");
  // println!("{}", *b)
  // boxdemo();
  
}