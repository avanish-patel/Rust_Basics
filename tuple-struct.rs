struct Color(u8, u8, u8);

fn main() {

    let c = Color(255,0,0);

    println!("{} {} {}", c.0, c.1, c.2);
}