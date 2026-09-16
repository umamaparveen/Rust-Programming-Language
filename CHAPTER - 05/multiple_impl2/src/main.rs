
struct Student {
    name : String,
    mark : u32,
}

impl Student {
    fn display(&self) {
        println!("Name : {}", self.name);
    }
}

impl Student {
    fn result(&self) -> bool {
        self.mark >= 40
    }
}

fn main() {
    let student = Student {
        name : String::from("Aalim"),
        mark : 70,
    };
    
    student.display();
    println!("{}", student.result());
}