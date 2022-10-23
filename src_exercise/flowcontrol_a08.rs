#![allow(unreachable_code)]
#![allow(unused_labels)]
#![allow(unused_variables)]

#[allow(dead_code)]
enum Color {
    Red, Blue, Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

struct Foo{
    x: (u32, u32),
    y: u32,
}

#[allow(dead_code)]
enum Temperature{
    Celsius(i32), Farenheit(i32),}

fn main() {

    // IF ELSE

    let n =5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive",n);
    } else {
        println!("{} is zero", n);
        }

    let big_n =
            if n < 10 && n > -10 {
                println!(", and is a small number, increase ten-fold");
                10*n
            } else {
                println!(", and isa  big number, halve the number");
                n/2
            };
    println!("{} -> {}", n , big_n);

    // LOOP

    let mut count = 0u32;
    println!("Lets count until infinity!");
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}", count);

        if count == 5 {
            println!("OK, thats enough");
            break;
        }
    }

    // NESTING LOP

    'outer: loop {
        println!("Entered the outer loop");
    'inner: loop {
        println!("Entered the inner loop");
        break 'outer;
    }
    println!("THis point will never be reached");
    }

    // Returning from loops

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }

    };
    println!("Counter is {}", counter);
    assert_eq!(result, 20);

    // WHILE

    let mut n = 1;
    while n < 101 {
        if n% 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n +=1;
    }

    // for and range
    // for and iterators

    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // MATCH

    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This si a prime"),
        12..=19 => println!("A teen"),
        _ => println!("Aint special"),
    };

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    // DESTRUCTURING TUPLES

    let triple = (0,-2,3);
    println!("Tell me about {:?}", triple);
    match triple {
        (0,y,z) => println!("First is `0`, `y`, is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesnt matter"),
        (.., 2) => println!("last is `2` and the rest doesnt matter"),
        (3, .., 4) => println!("First is `3`, last is `4`, and the rest doesnt matter"),
        _ => println!("It diesnt matter what they are"),
    }

    // DESTRUCTURING ARRAY SLICES

    let array = [1, -2, 6];
    match array {
        [0, second, third] => println!(
                "array[0] = 0, array[1] = {}, array[2] = {}", second, third ),
        [1, _, third] => println!(
                "array[0] = 1, array[2] = {} and array[1] was ignored", third),
        [-1, second, ..] => println!(
                "array[0] = -1, array[1] = {} and all the other ones were ignored", second),
        [3, second, tail @ ..] => println!(
                "array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        [first, middle @ .., last] => println!(
                "array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    }

    // DESTRUCTURING ENUMS

    let color = Color::RGB(122,17,40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red"),
        Color::Blue => println!("The color is Red"),
        Color::Green => println!("The color is Red"),
        Color::RGB(r,g,b) => println!(
                "Red: {}, green: {}, and blue: {}!", r,g,b),
        Color::HSV(h,s,v) => println!(
                "Hue: {}, saturation: {}, and value: {}!", h,s,v),
        Color::HSL(h,s,l) => println!(
                "Hue: {}, saturation: {}, and lightness: {}!", h,s,l),
        Color::CMY(c,m,y) => println!(
                "Cyan: {}, magenta: {}, and yellow: {}!", c,m,y),
        Color::CMYK(c,m,y,k) => println!(
                "Cyan: {}, magenta: {}, yellow {} and key(black): {}!", c,m,y,k),
    }

    // DESTRUCTURING POINTER/REF
    let reference = &4;
    match reference {
        &val => println!(
                "Got a value via destructuring: {:?}", val),
    }
    let _not_a_reference =3;
    let ref _is_a_reference =3;
    let value =5;
    let mut mut_value =6;

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10, 'mut_value': {:?}",m);
        },
    }

    // DESTRUCTURING STRUCTS

    let foo = Foo {x: (1,2), y:3};
    match foo {
        Foo{x: (1,b), y} => println!("First of x is 1, b = {}, y = {}",b,y),
        Foo {y: 2, x:i} => println!("y is 2, i = {:?}",i),
        Foo {y, ..} => println!("y = {}, we dont care about x",y),
    }

    // GUARDS

    let temperature = Temperature::Celsius(35);
    match temperature {
        Temperature::Celsius(t) if t > 30 => println!(
                "{}C is above 30 Cecius", t),
        Temperature::Celsius(t) => println!(
                "{}C is below 30 Cekcius",t),
        Temperature::Farenheit(t) if t > 86 => println!(
                "{}F is above 86 Farenheit",t),
        Temperature::Farenheit(t) => println!("{}F is below 86 Ferenheit", t),
    }

    // BINDING

    println!("Tell me what type of person you are");
    match age() {
        0 => println!("I have not celebrated my first brothday yet"),
        n @ 1 ..= 12 => println!("Im a child of age {:?}",n),
        n @ 13 ..= 19 => println!("Im a teen of age {:?}",n),
        n => println!("Im an adult of age {:?}",n),
    }

    // IF LET

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    if let Some(i) = number {println!("Matched {:?}", i);
    }
    if let Some(i) = letter {println!("Matched {:?}.", i);
    } else {
        println!("Didnt match a number. Lets go with a letter!");
    }
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}",i);
    } else if i_like_letters {
        println!("Didnt match a number. Lets go with a letter!");
    } else {
        println!("I dont like letters. Lets go with an emoticon :)!");
    }

    // WHILE LET

    let mut optional = Some(0);

    while let Some(i) = optional {
                if i > 9 {
                    println!("Greater than 9, quit");
                    optional = None;
                } else {
                    println!("'i' is {:?}'. Try again",i);
                    optional = Some(i+1);
                }
            }

    // CLOSNIG BRACKETS
}

fn age() -> u32 {15}