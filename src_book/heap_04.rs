fn type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    println!("Hello, world!");
    
    let s = "hello"; // string literal
    type_of(&s); 
    let mut s = String::from("hello"); // convert from string lieral
    type_of(&s); 
    s.push_str("world"); // push_str() appends a literal to a string
    type_of(&s);  
    let s = String::from("hello"); // memory return after s is used
    type_of(&s); 

    let s1 = String::from("hello"); 
    type_of(&s1); 
    let _s2 = s1;
    // println!("{}, world!", s2);

    // clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!(" s1 = {}. s2 = {}", s1, s2);

    // ownership and borowwing

    let v = String::from("Hello V");
    takes_ownership(v);
    let w = 5;
    makes_copy(w);
    type_of(&w);

    // Return Values and Scope

    let _a = gives_ownership();
    let b = String::from("hello");
    let _c = takes_and_gives_back(b);

    let d = String::from("hello");
    let (e, len) = calculate_length(d);
    println!("The length of '{}' is {}", e, len);

    // Reference and Borowwing

    let g = String::from("hello");
    let len2 = calculate_length_2(&g);
    println!("The length of '{}' is {}.", g, len2);

    // Mutable References
    let mut h = String::from("hello");
    change(&mut h);
    let len3 = calculate_length_3(&h);
    println!("The length of '{}' is {}.", h, len3);

    let mut q = String::from("hello");
    let r1 = &q;
    let r2 = &q;
    println!("r1 is {} and r2 is {}", r1, r2);
    let r3 = &mut q;
    println!("r3 is {}", r3); 

    // Dangling Referemce

    let string = dangle();
    println!("string is {}", string);

    // Slice Type
    let mut k = String::from("Hello World!");
    let word = first_word(&k);
    print!("The first word is {}", word);
    k.clear();
    
}

fn first_word(k: &String) -> &str {
    let bytes = k.as_bytes();
    for (_l, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &k[..];
        }
    }
    &k[..]
}

// Sting Literals Are Slices




fn takes_ownership(some_string: String) {
    println!("{}", some_string);

}

fn makes_copy(some_integer: i32) {
    print!( "{} \n", some_integer);
}

fn gives_ownership() -> String {
    let some_string_2 = String::from("yours");
    return some_string_2;
    
}

fn takes_and_gives_back(a_string: String) -> String {
    return a_string;
    
}

fn calculate_length(f: String) -> (String, usize) {
    let len1 = f.len();
     return (f, len1);
}

fn calculate_length_2(h: &String) -> usize {
    h.len()
}

fn calculate_length_3(g: &String) -> usize {
    g.len()
}

fn change(some_string_2: &mut String) {
    some_string_2.push_str(", World");
}

fn dangle() -> String {
    let j = String::from("hello");
    return j;
}