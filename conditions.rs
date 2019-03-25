
fn main() {

    let age: u8 = 11;


    if age > 18 {
        println!("You can vote.");
    }else if age == 18 {
        println!("You can apply for voter card.");
    }else {
        println!("You are too small, yet.");
    }
}