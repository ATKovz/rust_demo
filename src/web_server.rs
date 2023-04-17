use std::{
  net::{
    TcpListener,
    TcpStream
  },
  io::{BufReader, prelude::BufRead, Write, Read,},
  fs, fmt::format, ffi::c_long
};

fn read_file (file_path: &str, mut target: Vec<u8>) -> Vec<u8> {
  let target_file = format!("{}{}", '.', file_path);
  println!("{}", target_file);
  let r = fs::read(target_file);
  match r {
    Ok(s) => {
      for b in s.bytes() {
        let u = b.unwrap();
        target.push(u.to_ascii_lowercase());
      }
    },
    Err(err) => {
      let error_list = String::from("unknown file").as_bytes().to_vec();
      target = error_list;
    }
  }
  target
}

fn handle_connection (mut s: TcpStream) {
  let r = BufReader::new(&mut s);
  // buffer to string iter
  let result: Vec<_> = 
    r.lines()
    .map(|r| r.unwrap())
    .take_while(|x| !x.is_empty())
    .collect();

  let mut head = "HTTP/1.1 200 OK\r\n\r\n";

  let req_head = &result[0];
  let split_str: Vec<&str> = req_head.split(' ').map(|e| e).collect();

  if let [method, path, ver] = split_str[..] {
    println!("method: {}, path: {}, ver{}", method, path, ver);
    let target: Vec<u8> = vec![];
    
    let file_str = String::from_utf8(read_file(path, target)).unwrap();
    let result_bytes = file_str.as_str().as_bytes();
    // let head_bytes = head.to_ascii_uppercase().as_bytes();
    let head_bytes = head.as_bytes();
    let j  = [head_bytes, result_bytes].concat();
    s.write_all(&j);
    println!("{:?}", &j);
    println!("{:?}", String::from_utf8(j));
  } else {

    let content = "unknown file".as_bytes();
    let reture_item = [head.as_bytes(), content].concat();
    s.write_all(&reture_item);
    println!("{:?}", String::from_utf8(reture_item));
  }

  // s.write_all(response.as_bytes());
}

pub fn main () {
  let listener = TcpListener::bind("127.0.0.1:8082").unwrap();
  println!("rust server");
  for stream in listener.incoming() {
    let mut stream = stream.unwrap();
    handle_connection(stream);
  }

}