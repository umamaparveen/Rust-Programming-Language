pub struct Student {
    pub name : String,
    pub age : u32, 
}

pub fn display_student(student: &Student) {
    println!("Name : {}", student.name);
    println!("Age : {}", student.age);
}