fn main() {

    let vector = vec![1,2,3,4];

    for v in vector.iter() {
        println!("{}", v);
    }


    let mut mutable_vector: Vec<i32> = Vec::new();

    mutable_vector.push(45);
    mutable_vector.push(12);
    mutable_vector.push(63);
    mutable_vector.push(23);

    mutable_vector.remove(1); // remove element at index 1

    for v in mutable_vector.iter() {
        println!("{}", v);
    }
}