/*fn square(number: i32) -> i32 {
    number * number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(5), 25);
    }
}

  TESTING PRIVATE FUNCTIONS 

fn is_positive(number: i32) -> bool {
    number > 0 
}

#[cfg (test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_positive() {
    assert!(is_positive(10));
    assert!(is_positive(-5));
    }
} 
*/

fn calculate_discount(price: f64) -> f64 {
    price * 0.9
}

#[cfg (test)]
mod bill {
    use super::*;

    #[test]
    fn test_calculate_discount() {

    assert_eq!(calculate_discount(100.0), 90.0);
    assert_eq!(calculate_discount(200.0), 180.0);
    }
}
