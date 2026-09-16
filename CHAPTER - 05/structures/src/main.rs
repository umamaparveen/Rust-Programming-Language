struct Person(String, u32);

fn main() {
    let  student = Person(String::from("Sara"), 19);

 println!("{}", student.0);
 println!("{}", student.1);
}

