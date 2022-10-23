fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x,y);
}

fn failed_borrow<'a>() {
    let _x = 12;
    let _y: &'a i32 = &_x;
}

fn main() {

    // Explicit Annotations

    let i = 3;
    {
        let borrow1 = &1;
        println!("borrow1: {}", borrow1);
    }
    let borrow2 = &i;
    println!("borrow2: {}", borrow2);

    // Explicit Annotation
    let (four, nine) = (4,9);
    print_refs(&four, &nine);
    failed_borrow();

}