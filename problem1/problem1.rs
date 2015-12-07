use std::fs::File;
use std::io::prelude::*;
fn main(){
	use std::io::prelude::*;
	use std::fs::File;
	let mut f = File::open("input.txt").unwrap();
	let mut s = String::new();
	f.read_to_string(&mut s);
	let mut output = 0;
  for ch in s.chars(){
    match ch{
      '(' => output += 1,
      ')' => output -= 1,
      _ => output += 0
      }
  }
  println!("{}",output);
}
