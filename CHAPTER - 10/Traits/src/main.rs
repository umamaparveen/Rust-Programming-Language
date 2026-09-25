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

fn say_hello(person: impl Greet) {
    person.greet();
}

fn create_student() -> impl Greet {
    Student
}

fn main() {
    let student = create_student();
    student.greet();

    let teacher = Teacher;

    say_hello(teacher);
}