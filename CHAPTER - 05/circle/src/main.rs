struct Circle{
    radius : u32,
}

impl Circle{
    fn double_radius(&self) -> u32 {
        self.radius * 2
    }
}

fn main() {
let circle = Circle {
    radius : 5,
};

println!("{}", circle.double_radius());
}