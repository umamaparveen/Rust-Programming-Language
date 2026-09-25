/*------------------------------------CHAPTER - 10, PRACTICE---------------------------------*/

EXERCISE: 1

fn show_value<T:std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

fn main() {
    show_value(100);
    show_value(String::from("Rust"));
    show_value(25.5);
}

/*-------------------------------------------------------------------------------------------*/

EXERCISE: 2

trait Greet {
    fn greet(&self);
}

struct Student;
struct Teacher;

impl Greet for Student {
    fn greet(&self) {
        println!("Hello, I am a student");
    }
}

impl Greet for Teacher {
    fn greet(&self) {
        println!("Hello, I am a teacher");
    }
}

fn say_hello<T: Greet>(person: T) {
    person.greet();
}

fn main() {
    let student = Student;
    let teacher = Teacher;

    say_hello(student);
    say_hello(teacher)
}

/*-------------------------------------------------------------------------------------------*/

EXERCISE: 3

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn show(&self) {
        println!("This container has a value.");
    }
}

fn main() {
    let number = Container { value: 50 };
    let text = Container { value: String::from("Rust") };

    number.show();
    text.show();
    
}

/*-------------------------------------------------------------------------------------------*/

EXERCISE: 4

trait Storage {
    type Item;

    fn get(&self) -> Self::Item;
}

struct TextStorage {
    value: String,
}

impl Storage for TextStorage {
    type Item = String;

    fn get(&self) -> String {
    self.value.clone()
}
}

fn main() {
    let text = TextStorage {
    value: String::from("Rust is awesome"),
};

let result = text.get();

println!("{}", result);
}

/*-------------------------------------------------------------------------------------------*/

EXERCISE: 5

fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("Rust");
    let y = String::from("Programming");

    let result = shortest(&x, &y);

    println!("{}", result);
}