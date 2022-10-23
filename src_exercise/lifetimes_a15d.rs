use std::fmt::Debug;

// BOUNDS

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T) where
    T: Debug{
    println!("print': t is {:?}",t);
}

fn print_ref<'a, T> (t: &'a T) where
    T: Debug + 'a {
        println!("print_ref': t is {:?}", t);
    }

// COERCION

fn multiply<'a> (first: &'a i32, second: &i32) -> i32 {
    first * second
}

fn choose_first<'a: 'b, 'b> (first: & 'a i32, _:  &'b i32) -> &'b i32 {
    first
}

// Reference Lifetime
static NUM: i32 =18;
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

// ELISION

fn elided_input(x: &i32) {
    println!("elided_input: {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("'annotated_input' : {}", x);
}

fn elided_pass(x: &i32) -> &i32 {x}
fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {x}

fn main() {

    // BOUNDS
    let x =7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(ref_x);

    // COERCION

    let first = 2;
    {
        let second =3;
        println!("The prodct is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }

    // Static

    {
        let static_string = "Im the read-only memory";
        println!("static_string: {}", static_string);
    }
    {
        let lifetime_num =9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);
    }
    println!("NUM: {} stays accessible!", NUM);

    // ELISION

    let x =3;
    elided_input(&x);
    annotated_input(&x);
    println!("'elided_pass': {}", elided_pass(&x));
    println!("`annotated_pass`: {}", annotated_pass(&x));

}