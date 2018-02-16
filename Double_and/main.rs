#![feature(plugin)]
#![plugin(oak)]

extern crate oak_runtime;
//use oak_runtime::*;

grammar! double_and {
  #![show_api]
  test = &(&"a")
  test2 = &test
}

fn main() {
  println!("ok");
}
