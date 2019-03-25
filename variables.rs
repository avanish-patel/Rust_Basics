
fn main() {

    // can not mutate the variable
    let x = 45;

    println!("x = {}", x);

    // can mutate the variable
    let mut y = 30;

    println!("y = {}", y);

    y = 60;

    println!("y = {}", y);
}