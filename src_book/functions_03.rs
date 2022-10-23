fn main() {
    println!("Hello, world!");
    another_function(7);
    print_labeled_measurement(5, 'h');
    let b = {
        let
         c = 3;
        c+1
    };
    println!("The value of b is {b} and the calue of c is __");

    let d = five();
    println!("The value of d is {d}");

    let e = plus_one(5);
    println!("The value of e is {e}");

}

fn another_function(a: i32){
    println!("The value of a is: {a}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement of {value} is: {unit_label}");
}

fn five() -> i32{5}

fn plus_one(e:i32) -> i32{e+1}