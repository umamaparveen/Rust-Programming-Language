1. Sara
     20

2. ..student1 means it takes fields from student1. So that we don't need to write it again.

3. struct Product {
name: String
price: u32
quantity: u32
}

fn main() {
let product1 = Product {
name : String::from("Biscuit"),
price: 10,
quantity: 2,
};

let product2 = Product {
name : String::from("Chocolate"),
..product1
};

println!("{}", product2.name);
println!("{}", product2.price);
println!("{}", product2.quantity);
}

