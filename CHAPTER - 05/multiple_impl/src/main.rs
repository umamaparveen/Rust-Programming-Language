struct Car {
    brand : String,
    speed : u32,
}

impl Car {
    fn display(&self) {
    
        println!("Brand : {}", self.brand);
        println!("Speed : {}", self.speed);
    }
}

impl Car {
    fn is_fast(&self) -> bool {
     self.speed > 100
    }
}

fn main() {
    let car = Car {
        brand : String::from("Toyota"),
        speed : 120,
    };

    car.display();
    println!("{}", car.is_fast());
}