struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {

    fn print_rectangle(&self) {
        println!("Rectangle: {} X {}", self.width, self.height);
    }


    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {

    let rectangle = Rectangle{ width: 43, height:23};

    rectangle.print_rectangle();
    println!("Rectangle is Square : {}",rectangle.is_square());
}