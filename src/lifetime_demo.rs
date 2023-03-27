

pub fn main() {
  let b = "b";
  let s = "s";
  let r = join(s, b);
  let d = &s;
  println!("{}", r);
}

// 不能把生命周期在函数内的变量返回（dangle reference，悬挂引用）, 这是rust铁则
fn join <'aa>(a: &'aa str, b: &str) -> &'aa str {
  let x = String::from("abc") + a;
  x.as_str()
}

// 只有一个参数时，rust 编译器会自动加生命周期
fn longest_1(x: &str) -> &str {
  x
}
// 当存在2个参数时，rust编译器不知道他们的生命周期，所以要显式声明
fn longest<'a>(x: &'a str, y: &str) -> &'a str {

  x
}

// 字符串字面量隐性拥有 ‘static 生命周期 也就是和程序一致，因为编译后是二进制常量