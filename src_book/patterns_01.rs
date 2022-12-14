struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn main() {
    println!("Hello, world!");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("Using your favourite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is greenday!");
    } else if let Ok(age) = age {
        if age > 30 {
            println! ("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        } 
    } else {
        println!("Using blue as the background color");
        }

        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }

    let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index  {}", value, index);
        }

        // #####

        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}",x);

        // #####

        let p = Point {x: 0, y: 7};
        let Point {x,y} = p;
        assert_eq!(0, x);
        assert_eq!(7, y);

        // #####

        let msg = Message::ChangeColor(Color::Hsv(0,160,255));

        match msg {
            Message::ChangeColor(Color::Rgb(r,g,b)) => println! (
                "Change the color to red {}, green {}, and blue{} ",
                r,g,b
            ),
            Message::ChangeColor(Color::Hsv(h,s,v)) => println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,s,v
            ),
            _ => (),
         }
        
         let mut setting_value = Some(5);
         let new_setting_value = Some(10);

         match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Cant overwrite an existing customised value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }
        println!("setting is {:?}", setting_value);

        // #####

        // Closure Brackets
        }

