
use school::student::study;

mod school {
    pub mod student {
        pub fn study() {
            println!("Studying");
        }
    }
}

fn main() {
    study();
}