#[allow(dead_code)]

#[derive(Debug)]
enum UsState { Alabama, Alaska,}

enum Coin{ Quarter(UsState)}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // Coin::Penny => 1,
        // Coin::Nickel => 5,
        // Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25 }
    }
}

fn main() {

    value_in_cents(Coin::Quarter(UsState::Alaska));

    // match

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some (5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // catch all patterns

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
        // other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
    // fn move_player(num_spaces: u8) {}

    // If let

    // let coin = Coin::Penny;
    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!" , state);
    // } else {
    //     count += 1;
    //     }
    // }

    // Enums

    enum IpAddrKind { V4, V6}

    enum Message { 
        // Quit,
        // Move {x: i32, y: i32},
        Write(String),
        // ChangeColor(i32, i32, i32)
        
    }


    impl Message {
        fn call(&self) {
            println!("Hello, world!");
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    struct IpAddr {
        #[allow(dead_code)]
        kind: IpAddrKind,
        #[allow(dead_code)]
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),

    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

}

