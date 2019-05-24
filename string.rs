fn main() {

    let mut sentence = String::from("This is string in Rust programming.");

    println!("Length : {}", sentence.len());
    println!("Is Empty : {}", sentence.is_empty());

    for word in sentence.split_whitespace() {
        println!("{}", word);
    }

    println!("Contains Rust : {}", sentence.contains("Rust"));

    sentence.push_str(" Some more string!");

    println!("{}",sentence);

}