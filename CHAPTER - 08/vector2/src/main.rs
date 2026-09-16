fn main() {

let numbers = vec![5, 10, 15, 20, 25];

println!("{}", numbers[0]);
println!("{}", numbers[2]);

match numbers.get(20) {
Some(value) => println!("{}", value),
None => println!("There is no value"),
}

for number in &numbers {
println!("{}", number);
}
}