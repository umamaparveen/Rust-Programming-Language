fn main() {

    let word = "Rust".to_string();

    for character in word.chars() {
        println!("{}", character);
    }

    println!("{}", word.chars().count());
    println!("{}", word.len());
}
