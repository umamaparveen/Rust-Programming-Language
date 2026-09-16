use std::fs::File;

fn main() {
    let result = File::open("hello.text");

    match result{

        Ok(_file) => {
            println!("File opened successfully!");
        }

        Err(_) => {
            println!("File does not exist.");

            let _file = File::create("hello.txt");

            println!("File created!")
        }
    }
}