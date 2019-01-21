extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
  match n {
    0 => 0,
    1 => 1,
    _ => fib(n-2) + fib(n-1),
  }
}
