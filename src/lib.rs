#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate metrohash;
extern crate napi;
use std::mem::{self, size_of};
use std::fmt::Write;
use std::hash::Hasher;
use metrohash::{MetroHash128, MetroHash64};

fn hexlify<T>(value: &T) -> String {
  let mut result = String::with_capacity(size_of::<T>());
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

#[napi]
pub fn metrohash64(data: String, seed: u32) -> String {
  let mut hasher = MetroHash64::with_seed(u64::from(seed));
  hasher.write(data.as_bytes());
  let hash = hasher.finish();
  return hexlify(&hash);
}
