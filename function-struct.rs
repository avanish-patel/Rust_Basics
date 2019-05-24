struct Color {
    red: u8,
    blue: u8,
    green: u8
}


fn print_color(c: &Color) {
    println!("Red:{} Blue:{} Green:{}", c.red, c.blue, c.green);
}

fn main() {

    let c = Color { red: 34, blue:23, green: 45};

    print_color(&c);
    // Ownership still remain here, becuase passing by reference
    print_color(&c);
}

