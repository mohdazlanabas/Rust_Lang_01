fn function(){
    println!("called 'function()' ");
}

fn main() {
    my::function();
    function();
    my::indirect_acccess();
    my::nested::func();
}