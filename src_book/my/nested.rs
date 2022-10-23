pub fn function() {
    println!("called 'my::nested::function()' ");
}

#[allow(dead_code)]
fn private_function() {
    println!("called 'my::Nested::private_function()' ');")
}
