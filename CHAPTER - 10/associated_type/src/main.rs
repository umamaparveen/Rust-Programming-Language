trait Storage {
    type Item;

    fn get(&self) -> Self::Item;
}

struct NameStorage {
    name: String,
}

impl Storage for NameStorage {
    type Item = String;

    fn get(&self) -> Self::Item {
        self.name.clone()
    }
}

fn main() {
    let name1 = NameStorage {name: String::from("Umama")};
    let result = name1.get();

    println!("{}", result);
}
