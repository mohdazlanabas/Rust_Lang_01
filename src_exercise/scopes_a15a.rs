struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn create_box() {
    let _bool = Box::new(3i32);
}

fn main() {
    let _bool2 = Box::new(5i32);
    {let _bool3 = Box::new(4i32);}
    for _ in 0u32..1000 {
        create_box();
    }

    let _x = ToDrop;
    println!("Made a ToDrop!");


    // CLOSING BRACES
}