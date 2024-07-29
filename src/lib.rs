#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate metrohash;
extern crate napi;
use std::mem;
use std::fmt::Write;
use std::hash::Hasher;
use metrohash::MetroHash128;

fn hexlify(value: &(u64, u64)) -> String {
  let mut result = String::with_capacity(16);
  unsafe {
      let value_bytes: *const u8 = mem::transmute(value);
      for i in 0..result.capacity() {
          write!(result, "{:02x}", *value_bytes.offset(i as isize)).unwrap();
      }
  }
  result
}

#[napi]
pub fn metrohash128(data: String, seed: u32) -> String {
  let mut hasher = MetroHash128::with_seed(u64::from(seed));
  hasher.write(data.as_bytes());
  let hash = hasher.finish128();
  return hexlify(&hash);
}
