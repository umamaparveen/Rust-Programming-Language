
    fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Cannot divide by zero"));
    }

    Ok(a / b)
}


#[test]
fn test_divide() -> Result<(), String> {
let number = divide(10, 2)?;
assert_eq!(number, 5);

Ok(())
}
