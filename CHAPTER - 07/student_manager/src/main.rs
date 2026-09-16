mod student;
mod teacher;
mod school;

use school::{
    Student,
    display_student,
    Teacher,
    display_teacher,
};

fn main() {
    let student = Student {
        name : String::from("Aisha"),
        age : 21,
    };

    let teacher = Teacher {
        name : String::from("Rahman"),
        subject : String::from("Rust"),
    };

    display_student(&student);
    display_teacher(&teacher);
}
