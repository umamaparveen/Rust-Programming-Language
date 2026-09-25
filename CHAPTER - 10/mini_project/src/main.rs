trait Info {
    fn show_info(&self);
}

struct Student {
    name: String,
    age: u32,
}

struct Teacher {
    name: String,
    subject: String,
}

struct DataManager {
    items: Vec<Box<dyn Info>>,
}

impl Info for Student {
    fn show_info(&self) {
        println!("Student: {}, Age: {}", self.name, self.age);
    }
}

impl Info for Teacher {
    fn show_info(&self) {
        println!("Teacher: {}, Subject: {}", self.name, self.subject);
    }
}

impl DataManager {
    fn add(&mut self, item: Box<dyn Info>) {
        self.items.push(item);
    }

    fn show_all(&self) {
        for item in &self.items {
            item.show_info();
        }
    }
}

fn main() {
    let mut manager = DataManager {
    items: Vec::new(),
};

    manager.add(Box::new(Student {
    name: String::from("Umama"),
    age: 21,
}));

manager.add(Box::new(Teacher {
    name: String::from("ChatGPT"),
    subject: String::from("Rust"),
}));

    manager.show_all();

}