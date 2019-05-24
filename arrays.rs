fn main() {

    let numbers = [14,83,38,8,81];

    for i in numbers.iter() {
        println!("{}",i);
    }

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }

    // array of type i32 and has length 6
    let another_numbers : [i32;6] = [1,2,3,4,5,6];
    println!("legnth : {}", another_numbers.len());

    // has 2 in array 10 times
    let homogenious_numbers = [2;10];

    for i in homogenious_numbers.iter() {
        println!("{}",i);
    }
}