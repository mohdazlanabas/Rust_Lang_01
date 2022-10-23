use std::time::SystemTime;
fn main() {
    println!("Rustlang count to One Billion");
    let x = timeit(|| counter());
    print!("{:?}" , x)}
fn counter () {
    let mut i:i64 = 0;
    while i < 1000000000 {i += 1;}
    println!("END");}
fn timeit<F: Fn() -> T, T>(f: F) -> T {
    let start = SystemTime::now();
    let result = f();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("it took {} seconds", duration.as_secs());
    println!("Rustlang is Better, 17 lines of codes");
    result}