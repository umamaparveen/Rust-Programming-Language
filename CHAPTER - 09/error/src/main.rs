fn check_age(age: i32) {

    if age < 18 {
        panic!("You must be 18 or older!");

    } else {

        println!("You can enter");
    }
    }

fn main() {

    check_age(15);
}
