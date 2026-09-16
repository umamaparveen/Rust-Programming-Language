use std::collections::HashMap;

fn main() {
    let mut students = vec![
        "Aisha",
        "John",
        "Rahman",
    ];

    let mut marks = HashMap::new();

    marks.insert(students[0], 90);
    marks.insert(students[1], 85);
    marks.insert(students[2], 95);

    match marks.get("Aisha") {
        Some(mark) => println!("{}", mark),
        None => println!("Mark not found"),
    }

    marks.insert(students[0], 95);

    if students.contains(&"John") {
        println!("John exists");
    }

    marks.remove("Rahman");

    println!("{:?}", students);
    println!("{:?}", marks);
}