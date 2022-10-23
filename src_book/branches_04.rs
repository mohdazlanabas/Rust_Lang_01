fn main() {
    println!("Hello, world!");
    let number =6;
    let (a,b,c): (i32, i32, i32) = (4,3,2);

    if number % a == 0 {
        println!("the number is divisible by 4");
    } else if number % b == 0 {
        println!("the number is divisible by 3");
    } else if number % c == 0 {
    println!("the number is divisible by 2");
    }else {
        println!("the number is not divisible by {a}, {b} or {c}");
    }

    let _number2: i32 = 9;
    let condition = true;
    let number2 = if condition{8} else {9};
    println!("number2 is {number2}");

    let _char01: char = 'e';
    let condition = true;
    let number3 = if condition{'e'} else {'f'};
    println!("number3 is {number3}");

    let mut counter = 0;
    let result = loop
    {
        counter +=1;
        if counter == 10 {
            break counter;
        }
        
    };
    println!("The result is {result}"); 

    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

    loop {
        println!("remaining: {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut  number3 = 3;
    while number3 != 0 {
        println!("{}", number3);
        number3 -=1;
    }
    println!("LIFTOFF"); 

    let a = [10,20,30,40,50];
    let mut index =0;
    while index < a.len() {
        println!("The value is: {}",  a[index]);
        index +=1;
    }   

    for ele in a {
        println!("The ele value is: {}", ele);
    }

    for counting in (1..5).rev() {
        println!("The counting down is: {}", counting);
    }
    println!("TOUCHDOWN"); 

}


