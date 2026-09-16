
2A:
fn main() {
    let age: Option<i32> = None;

    match age {
        Some(value) => println!("Age is 21"),
        None => println!("Age is not provided"),
    }
}


2B:
fn main() {
    let age = Some(21);

    if let Some(value) = age {
        println!("Age is {}",value);
    }
}
