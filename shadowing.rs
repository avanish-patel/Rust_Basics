
fn main() {

    // will chage the value , assignment in {}
    let mut x = 10;
    {
        x = 20;
    }
    println!("{}",x);

    // will not change value, assignment in {}
    let mut y = 10;
    {
        let y = 20;
    }
    println!("{}",y);
}