use std::collections::HashMap;

fn main() {

    let students = vec![
        "Aisha",
        "Rahman",
        "John",
    ];

    let mut marks = HashMap::new();

    marks.insert(students[0], 90);
    marks.insert(students[1], 85);
    marks.insert(students[2], 95);

    match marks.get("Aisha") {
        Some(mark) => println!("{}", mark),
        None => println!("Mark not found!"),
    }

    marks.insert(students[0], 95);

    println!("Students Mark: {:?}", marks);

}