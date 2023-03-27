use std::{collections::HashMap, hash};

pub fn main () {

  let mut hash_map = HashMap::new();

  hash_map.insert("name", String::from("teo"));

  // let s = str_impl.to_string();

  hash_map.entry("gender").and_modify(|s| { *s = "male".to_string(); });
  hash_map.entry("name").and_modify(|s| {
    *s = s.to_owned() + "__Tail";
  });

  

  println!("{:?} {:?}", hash_map, hash_map.get("name"));
}