pub struct Teacher {
    pub name : String,
    pub subject : String,
}

pub fn display_teacher(teacher:&Teacher) {
    println!("Teacher : {}", teacher.name);
    println!("Subject : {}", teacher.subject);
}