pub fn main () {

  let mut list = vec!["abc", "efg"];

  let last_child =  list.last();

  for afb in &list {
    println!("{}", afb)
  }
  // if let Some(v) = last_child {
  //   println!("{}", v);
  // }
  let res = vec!["1","2", "3", "4"];
  list.splice(1..2, res);
  println!("vec {:?} {}", list, list.capacity())
  
}