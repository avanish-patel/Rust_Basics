
fn main() {

    print_numbers_to(10);
    println!("{}", is_even(20));
    println!("{}", is_even(7));

}

fn print_numbers_to(number: u32) {

    for i in 1..number {
        print!("{} ", i);
    }
    println!();
}

fn is_even(number: u32) -> bool {

    return number % 2 == 0;
}