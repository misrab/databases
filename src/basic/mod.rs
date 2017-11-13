use std::fs;
use std::io::prelude::*;


static DB_FILE: &'static str = "db.txt";



// open db connection
pub fn open_connection() {
  let mut file = fs::File::create(DB_FILE);
}

// add the string to the file
pub fn db_set(s: &str) {
  let mut file = fs::File::open(DB_FILE).unwrap();

  if let Err(e) = writeln!(file, "{}", s) {
    println!("{}", e);
  }
}

// clear contents
pub fn delete_connection() {
  fs::remove_file(DB_FILE);
}


/*
#[test]
fn test_basic() {
  println!("test basic");


  open_connection();
}*/
