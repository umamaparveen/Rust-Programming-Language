fn main() {

let mut numbers : Vec<i32> = Vec::new();
numbers.push(10);
numbers.push(20);
numbers.push(30);
numbers.push(40);
numbers.push(50);

for number in &numbers {
println!("{}", number);
}
}