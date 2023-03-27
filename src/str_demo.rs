pub fn main () {

  let mut str_impl = String::from("string");

  // let s = str_impl.to_string();

  str_impl.push('c');

  let result = str_impl + "_tail";
  println!("{}", result);
}