
  // assert_eq! for testing is these values are equal? 

    fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(4,5), 20);
}  


 /* assert_ne! for testing that two values must not be equal 

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

#[test]
fn test_even() {
    assert_ne!(9487425);
} */


/*  assert! for checking conditions 

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

#[test]
fn test_even() {
    assert!(is_even(9487422));
}
*/


/*  #[should_panic] for declaring panic intensionally 

    fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }

    a / b
}

#[test]
#[should_panic]
fn test_divide_by_zero() {
    divide(10, 0);
} */

/* #[should_panic(expected = " ")] to know the reason for panic

    fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }

    a / b
}

#[test]
#[should_panic(expected = "Cannot didvided by 0")]
fn test_divide_by_zero() {
    divide(10, 0);
}
*/