use::std::collections::HashMap;

fn main() {

    let numbers = vec![1, 2, 1, 3, 2, 1, 4];

    let mut count = HashMap::new();

    for number in numbers {

        let entry = count.entry(number).or_insert(0);
        *entry += 1;
    }

    println!("{:?}", count);
}