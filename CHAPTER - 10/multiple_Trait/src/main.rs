trait Greet {
    fn greet(&self);
}

trait Introduce {
    fn introduce(&self);
}

struct Student;

impl Greet for Student {
    fn greet(&self) {
        println!("Hello!");
    }
}

impl Introduce for Student {
    fn introduce(&self) {
        println!("I am a student.");
    }
}

fn student_info<T>(student: T) 

where
T: Greet + Introduce
{
    student.greet();
    student.introduce();
}

fn main() {
    let student = Student;

    student_info(student);
}