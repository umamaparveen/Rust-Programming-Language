struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width : 5,
        height : 5,
    };

    println!("{}", rectangle.area());
    println!("{}", rectangle.is_square());
}
