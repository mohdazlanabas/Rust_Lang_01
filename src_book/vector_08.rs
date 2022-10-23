use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // let v = vec![0,1,2,3,4,5,6,7,8,9];

    // Vectors
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    println!("The Vector V is : {:?}", v);
    
    let third: &i32 = &v[2];
    println!("Third Element is: {}", third);

    let third: Option<&i32> = v.get(12);
    match third {
        Some(third) => println!("The third element is: {}", third),
        None => println!("The is no third element"),
        }

        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let _row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        
        // String

        // let mut s = String::new();
        let data = "initial contents";
        let _s = data.to_string();
        let s = "initial contents".to_string();
        println!("{}", s);

        let x1 = s;
        let pointer01 = &x1;
        println!("x1: {}; address: {:p} ", x1, pointer01);

        let x2 = data;
        let pointer01 = &x2;
        println!("x: {}; address: {:p} ", x2, pointer01);

        let mut t = String::from("foo");
        let t2 = "tang";
        t.push_str("bar");
        t.push_str(t2);
        println!("{}", t);

        let s1 = String::from("hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        println!("{}", s3);

        // Hasmap

        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("yellow"), 50);

        let team_name = String::from("Blue");
        let score1 = scores.get(&team_name).copied().unwrap_or(0);
        println!("{}", score1);

        for (key, value) in &scores {
            println!("{}:{}", key, value);
        }

        let field_name = String::from("Favourite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        scores.entry(String::from("Blue")).or_insert(120);
        scores.entry(String::from("Yellow")).or_insert(100);
        
        for (key, value) in &scores {
            println!("{}:{}", key, value);
        }
         
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count +=1;
        }
        println!("{:?}", map);

    }

