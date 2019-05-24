struct Color {
    red: u8,
    green: u8,
    blue: u8
}


fn main() {

    let c1 = Color { red:255, green:0, blue:0};
    println!("Red :{} Green:{} Blue:{}", c1.red, c1.green, c1.blue);


    let mut c2 = Color { red: 234, green:24, blue:12};
    c2.red = 33;
    c2.green = 33;
    c2.blue = 33;
    println!("Red:{} Green:{} Blue:{}", c2.red, c2.green, c2.blue);

}