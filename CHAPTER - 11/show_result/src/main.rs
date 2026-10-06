/*fn sum_one(a: i32, b: i32) -> i32 {
    a + b 
}

fn sum_two(c: i32, d: i32) -> i32 {
    c + d
}

#[test]
fn test_one() {
    println!("This is test one");
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_two() {
    println!("This is test two");
    assert_eq!(3 + 3, 6);
}*/
/* 
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_multiplication() {
    assert_eq!(3 * 3, 9);
}*/

#[test]
fn normal_test() {
    assert_eq!((5 + 5), 10);
}

#[test]
#[ignore]
fn ignored_test() {
    assert_eq!((5 * 5), 25)
}