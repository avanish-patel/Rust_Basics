
fn main() {

    // Imutable reference - can't modify the value it's pointing to
    let x = 40;
    let y = &x;

    println!("{}", y);

    // Mutable reference - can modify the value it's pointing to
    let mut a = 40;
    let b = &mut a;
    *b += 5;

    println!("{}", b);

    // Mutable reference inside scope to not to borrow twice - Modify value inside scope and then pass variable in println!
    let mut p = 40;
    {
        let q = &mut p;
        *q += 10;
    }

    println!("{}", p);
}