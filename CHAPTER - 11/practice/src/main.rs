/* fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg (test)]

mod test {
    use super::*;

#[test]
fn test_multiply() {
    assert_eq!(multiply(4, 5), 20);
    assert_eq!(multiply(10, 3), 30);
}
} 

EXERCISE - 2 

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn test_is_even() {
    assert!(is_even(20));
    assert!(!is_even(15));
}
}

      EXERCISE - 3 */

fn get_element(numbers: &[i32], index: usize) -> i32 {
    if index >= numbers.len() {
        panic!("Index out of bounds");
    }

    numbers[index]
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    #[should_panic]
    
    fn test_get_element() {
        get_element(&[10, 20, 30], 5);
    }
}
