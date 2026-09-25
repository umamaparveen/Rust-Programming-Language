trait Greet {
    fn greet(&self) {
        println!("Hello from the Greet trait!");
    }
}

struct Student;
struct Teacher;

impl Greet for Student {}

impl Greet for Teacher {
    fn greet(&self) {
        println!("Hello, I am a teacher");
    }
}


fn say_hello<T>(person: T)
where
    T: Greet,
{
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