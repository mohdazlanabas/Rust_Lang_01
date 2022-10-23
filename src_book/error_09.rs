use std::fmt;
use std::fs::File;
use std::io::{self, Read, Error};
use std::net::IpAddr;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    let v = vec![1,2,3];
    v[1];
    // println!(" v is {}", v);
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let greeting_file_results = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem opening the files: {:?}", e),
        },
        other_error => {
            panic!("Problem opening the files: {:?}", other_error);
            }
        },
    };

}

fn read_username_from_file() -> Result,String, io::Error> {
    let username_file_result = File::open("hello2.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => return Err(e),
    }
}

fn last_char_of_first_line(tect: &str) -> Optio<char> {
    text.lines().next()?.chars().last()
}
