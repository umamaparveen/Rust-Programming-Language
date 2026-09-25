fn display<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

fn return_value<T>(value: T) -> T {
    value
}

fn main() {
    display(10);
    display("Rust");
    display(6.4);

    let number = return_value(67);
    let text = return_value(String::from("Hello"));

    println!("Returned number: {}", number);
    println!("Returned text: {}", text);
}
                                                  
