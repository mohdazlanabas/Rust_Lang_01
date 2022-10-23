use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We are sorry, the call cannot be completed as dialed.
        Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Aweseome Pizza. My name is Fred.
        What can i get for you today?",
        _=> "Hi Who is this again?"
    }
}

fn main() {
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "876-1745");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", call(number)),
    _ => println!("Dont have Daniels number."),
}

    contacts.remove(&"Ashley");
    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }


}