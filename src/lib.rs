#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b + 1
}

#[napi]
pub fn sum2(a: i32, b: i32) -> i32 {
  a + b
}
