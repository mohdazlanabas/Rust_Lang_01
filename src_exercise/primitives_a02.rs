use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slize: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    println!("Hello, world!");

    // Integer
    println!(" 1 + 2 = {}", 1u32 + 2);
    println!("1 -2 = {}", 1i32 -2);

    // Boolean
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // underscre improve readibility
    let num: u32;
    num = 1_000_000;
    println!("One million is written as {}", num);

    // Bitwise operation
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // TUPLES

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                        -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64, "a", true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a,b,c,d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);

    let matrix = Matrix(1.2,1.2,2.1,2.1);
    println!("{:?}", matrix);

    // ARRAY AND SLICES

    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);
    println!("number of elements in array: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }


    // CLOSING BRACKETS
}
