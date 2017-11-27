use std::fs;
use std::io::prelude::*;


static DB_FILE: &'static str = "db.txt";

pub trait Database {
  fn summary(&self) -> String;
}

pub struct BasicDatabase {

}

impl Database for BasicDatabase {
  fn summary(&self) -> String {
    format!("{}", "I'm a basic database")
  }

}



// open db connection
pub fn open_connection() -> BasicDatabase {
  let mut file = fs::File::create(DB_FILE);

  let db = BasicDatabase{
  };

  db
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
