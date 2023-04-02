pub fn main () {
  let mut list = vec![1,2,3];

  // 不可行，let只能接受常规的不可失败值
  // let Some(val) = Some(123);


  // 不要这样写，这样写等于 if true
  if let val = 123 {
    println!("PASSED");
  }
  let x = 123;

  // 匹配值应该这样写
  match x {
    1 => {},
    2 => {},
    3 => {},
    4 | 5 => {},

    // 范围写法， 字符可以写 'a' ..= 'b'，匹配ascii
    6 ..= 10 => {},
    _ => {}
  }

  let s = 'c';
  match s {
    'a' ..= 'b' => {}
    name @'c' ..= 'd' => {
      // 把条件绑定到name这变量上
      println!("{}", name);
    }
    _ => {}
  }

  if let Some(val) = Some(123) {

  }

  while let Some(v) = list.pop() {
    println!("match {}", v);
  }

  struct A<'a> {
    name: &'a str,
    age: i32
  }
  struct RGB(i32, i32, i32);
  // struct 匹配
  let a = A{ name: "Teo", age: 32 };
  let A { name: a_name, age: a_age } = a;
  let rgb = RGB(34, 255, 255);
  let RGB(r, g, b) = rgb;

  match rgb {
    // _在很多场景都可以表示忽略
    RGB(12, _, _) => {},
    // ..也可以
    RGB(32, ..) => {},
    // 中间不行
    // RGB(.., 32, ..) => {},
    RGB(r@33..=50 , ..) if r > 25 => {
      // 命名的同时带有条件
      println!("{}", r)
    },

    // 匹配守卫，
    RGB(r, ..) if r > 32 => {},
    RGB(r, ..) if r > 25 => {},
    _ => {}
  }
}