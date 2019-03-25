
fn main() {

    for i in 1..11 {
        println!("{}", i);
    }

    let fruits = vec!["Apple","Banana","Orange"];

    for fruit in fruits.iter() {
        println!("{}", fruit);
    }


    for (index, fruit) in fruits.iter().enumerate() {

        println!("{} : {}", index, fruit);
    }
}