use std::mem;

fn apply<F>(f: F) where F: FnOnce() { f(); }
fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 { f(3)}
// fn call_me<F: Fn()>(f: F) { f();}
// fn function() { println!("Im a functioin");}
fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("THis is a: {}", text)
}
fn create_fnmut() -> impl FnMut() {
    let text = "FnNut".to_owned();
    move || println!("This is a: {}", text)
}
fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

//pub trait Iterator {
//    type Item;
//    fn any<F>(&mut self, f: F) -> bool where F: FnMut(Self::Item) -> bool;}

fn is_odd(n: u32) -> bool {
    n% 2 == 1
}

fn main (){

    // CLOSURES
    fn function(i:i32) -> i32 {i+1}
    let closure_annotated = |i:i32| -> i32 {i+1};
    let closure_inferred = |i| i+1;
    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    let one = || 1;
    println!("closure returning one: {}", one());

    // CAPTURING
    let color = String::from("green");
    let print = || println!("'color' : {}", color);
    print();
    let _reborrow = &color;
    print();
    let _color_moved = color;
    let mut count =0;
    let mut inc = || {
        count += 1;
        println!("'count': {}", count);
    };
    inc();
    let _count_reborrowed = &mut count;
    let movable = Box::new(3);
    let consume = || {
        println!("'movable': {:?}", movable);
        mem::drop(movable);
    };
    consume();

    // As Input Parameters

    let greeting = "hello";
    let mut farewell = "good bye".to_owned();
    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");
        mem::drop(farewell);
    };
    apply(diary);
    let double = |x| 2 *x;
    println!("3 doubled: {}", apply_to_3(double));

    // Anoynimity
    // let closure = || println!("Im a closure");
    // call_me(closure);
    //call_me(function);

    // OUTPUT PARAMATER
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain(); fn_mut(); fn_once();

    // Iterator::any
    //let vec1 = vec![1,2,3];
    //let vec2 = vec![4,5,6];
    //println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    //println!("2 in vec2: {}", vec2.into_iter().any(| x| x == 2));
    //println!("vec1 len: {}", vec1.len());
    //println!("First element of vec1 is: {}", vec1[0]);
    //let array1 = [1,2,3];
    //let array2 = [4, 5, 6];
    //println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    //println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;
    let mut acc =0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);
    let sum_of_squared_odd_numbers: u32 =
    (0..).map(|n| n * n)
    .take_while(|&n_squared| n_squared < upper)
    .filter(|&n_squared| is_odd(n_squared))
    .sum();
    println!("functional style: {}", sum_of_squared_odd_numbers);

    //CLOSING BNRACKETS
}