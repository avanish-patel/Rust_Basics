
fn main() {

    let tuple1 = (10,20,30,40);

    println!("{}", tuple1.0);
    println!("{}", tuple1.1);
    println!("{}", tuple1.2);
    println!("{}", tuple1.3);

    let tuple2 = (100, 4.832, false, (23, 48));

    println!("{}", tuple2.0);
    println!("{}", tuple2.1);
    println!("{}", tuple2.2);
    println!("{}", (tuple2.3).0);
    println!("{}", (tuple2.3).1);

    let (a,b,c,d) = tuple1;

    println!("{},{},{},{}", a,b,c,d);
}