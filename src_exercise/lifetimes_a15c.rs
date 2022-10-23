// structs

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
#[allow(dead_code)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

// Traits

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10,}
    }
}


fn main() {

    // structs

    let x = 10;
    let y = 15;
    let single = Borrowed(&x);
    let double = NamedBorrowed {x: &x, y: &y};
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);

    // Traits

    let b: Borrowed = Default::default();
    println!("b is {:?}",b);
}