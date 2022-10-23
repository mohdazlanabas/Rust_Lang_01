use deeply::nested::function as other_function;

fn function() {
    println!("called 'function()'");
}

mod cool {
        pub fn function() {
            println!("called 'cool::function()'");
        }
    }
mod my {
        fn function () {
            println!("called 'my::function()'");
        }
mod cool {
            pub fn function() {
                println!("called 'my::cool::function()'");
            }
        }

pub fn indirect_call() {
            print!("called 'my::indirect_call()', that\n> ");
            self::function();
            function();
            self::cool::function();
            super::function();
            {
                use crate::cool::function as root_function;
                root_function;
            }
        }
    }

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called 'deeply::nested::Function()'");
        }
    }
}

/*
mod my{
    pub struct OpenBox<T> {pub contents: T,}
#[allow(dead_code)]
pub struct ClosedBox<T> { contents: T,}

impl<T> ClosedBox<T>{
        pub fn new(contents: T) -> ClosedBox<T> {
    ClosedBox {
        contents: contents,
                }
            }
        }
    }
*/

mod my_mod {
    fn private_function() {
        println!("called 'my_mod::provate_function()'");
    }
    pub fn function() {
        println!("called 'my_mod::function()'");
    }
    pub fn indirect_access() {
        println!("called 'my_mod::indirect_access()', that\n> ");
        private_function();
    }
    pub mod nested {
        pub fn function() {
            println!("called 'my_mod::nested::private_function()'");
        }
        #[allow(dead_code)]
        fn private_function() {
            println!("called 'my_mod::nested::private_function()'");
        }
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called 'my_mod::nested::call_public_function_in_my_mod()', that\n> ");
            public_function_in_nested();
        }
        pub(self) fn public_function_in_nested() {
            println!("called 'my_mod::nested::public_function_in_nested()'");
        }
        pub(super) fn public_function_in_super_mod() {
            println!("called 'my_mod::nested::public_function_in_super_mod()' ");
        }
    }

pub fn call_public_function_in_my_mod() {
        println!("called 'my_mod::call_public_function_in_my_mod()', that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

mod private_nested {
    #[allow(dead_code)]
    pub fn function() {
        println!("called 'my_mod::private_nested::function()'");
    }
    #[allow(dead_code)]
    pub(crate) fn restrictrf_function() {
        println!("called 'my_mod::private_nested::restricted_function()'");
            }
        }
    }

fn main() {

    // let open_box = my::OpenBox{contents:"public information"};
    // let _closed_box = my::ClosedBox::new("classified information");
   //  println!("The open box contains: {}", open_box.contents);
    println!("Entering block"); {
        use crate::deeply::nested::function;
        function();
        println!("Leaving block");
    }

    other_function();
    function();
    my_mod::function();
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();
    my_mod::public_function_in_crate();
    my::indirect_call();
}