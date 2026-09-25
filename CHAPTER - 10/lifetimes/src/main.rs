struct Book<'a> {
    title: & 'a str,
}

fn main() {
    let title = String::from("The Rust Book");

    let book = Book {
        title: &title,
    };

    println!("{}", book.title);
}

/* fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let first = String::from("Hello");
    let second = String::from("Hello Rust programming");

    let result = longest(&first, &second);

    println!("Longest: {}", result);
} */

 