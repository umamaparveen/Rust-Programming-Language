use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let result = File::open("hello.txt");

    match result {
        Ok(_file) => {
            println!("File opened successfully!");
    }

    Err(error) => {

        match error.kind() {

            ErrorKind::NotFound => {

        println!("File does not exist. Creating it...");

        let _file = File::create("hello.txt");

        println!("File created successfully.");
       }

       ErrorKind::PermissionDenied => {

        println!("You don't have permission to open this file");
       }

       _=> {

       println!("Something went wrong");
        }
      }
    }
  }
}