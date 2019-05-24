struct Person {
    name: String,
    age: u8
}

impl ToString for Person {
    
     fn to_string(&self) -> String {
        return format!("My name is {} and I'm {} years old.", self.name, self.age);
     }
}

fn main() {

    let person  = Person { name: String::from("Avanish"), age:25};

    println!("{}", person.to_string());

}