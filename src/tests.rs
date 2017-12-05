#![feature(test)]


extern crate test;

use basic;


#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;



  #[test]
  fn test_test() {
    println!("tests are running");
  }



  #[test]
  fn test_basic() {
    println!("test basic");

    basic::open_connection();
    basic::db_set("mooo");
    basic::delete_connection();
  }



  fn bench_insert(b :&mut Bencher) {

  }
}
