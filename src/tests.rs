
use basic;


#[cfg(test)]
mod test {
  use super::*;




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

}
