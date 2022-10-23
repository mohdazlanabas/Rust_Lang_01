// DROP

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
        fn drop(&mut self) {
            println!("> Dropping {}", self.name);
        }
    }

// ITERATORS

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        self.curr =self.next;
        self.next = current + self.next;
        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {curr: 0, next: 1}
}

fn main() {

    // DROP
        let _a = Droppable {name: "a"};
        { let _b = Droppable {name: "b"};
            {
                    let _c = Droppable {name: "c"};
                    let _d = Droppable {name: "d"};
                    println!("Exiting block B");
                }
                println!("Just exited block B");
                println!("Exiting block A");
            }
            println!("Just exited block A");
            drop(_a);
            println!("end of the main function");

            // ITERATOR

    let mut sequence = 0..3;
            println!("Four consecutive 'next' calls on 0..3");
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
            println!("> {:?}", sequence.next());
            println!("Iterate through 0..3 using 'for'");
            for i in 0..3 {
                println!("> {}", i);}

    println!("The first four terms of the Fibonacci sequence are: ");
            for i in fibonacci().take(4) {
                println!("> {}",i);}
    println!("The next four terms of the Fibonacci sequence are: ");
            for i in fibonacci().skip(4).take(4) {
                println!("> {}",i);}
    let array = [1u32,3,3,7];
    println!("Iterate the following array {:?}", &array);
            for i in array.iter() {
                println!("> {}",i);}

            // CLOSING BRACKETS
        }
