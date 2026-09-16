fn main() {
    let numbers = [10,20,30,40,50];
    show_slice(&numbers[1..4]);
}
fn show_slice(numbers: &i32){
println!("{numbers}");
}