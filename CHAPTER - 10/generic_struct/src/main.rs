struct BoxValue<T, U, V> {
    first: T,
    second: U,
    third: V,
}

fn main() {
    let boxvalue = BoxValue {
        first: 100,
        second: String::from("Rust"),
        third: 100.12,
    };

    println!("{}", boxvalue.first);
    println!("{}", boxvalue.second);
    println!("{}", boxvalue.third);
}