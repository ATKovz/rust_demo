

// 2种写法效果一样
// fn get_max <T:PartialOrd + Copy>(list: &Vec<T>) -> T  {
fn get_max <T>(list: &Vec<T>) -> T where T:PartialOrd + Copy  {
  let mut max = list[0];
  for &i in list.iter() {
    if max < i {
      max = i;
    } 
  }
  max
}

pub fn main () {
  let list_num = vec![1,2,2,3,43,5,2345,23,45,234, 3];
  let list_str = vec!["asd", "bb"];
  println!("{}", get_max(&list_num));
  println!("{}", get_max(&list_str));
}