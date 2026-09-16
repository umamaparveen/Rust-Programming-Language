fn main() {

    use::std::collections::HashMap;

    let mut marks = HashMap::new();

    marks.insert("Math", 90);
    marks.insert("Rust", 95);
    marks.insert("DBMS", 85);

    match marks.get("Rust") {
        Some(marks) => println!("Rust : {}", marks),
        None => println!("Mark not found"),
    }

    for(subject, mark) in &marks {
        println!("{} : {}", subject, mark);
    }
}