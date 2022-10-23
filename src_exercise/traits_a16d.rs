// impl traits

use std::iter;
use std::vec::IntoIter;

#[allow(dead_code)]
fn combine_vecs_explicit_return_type(
        v: Vec<i32>,
        u: Vec<i32>,
        ) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn combine_vecs(
        v: Vec<i32>,
        u: Vec<i32>,
        ) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Clone

#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

// Super TRaits
trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

#[allow(dead_code)]
fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
            )
}

fn main() {

    // impl traits

    let v1 = vec![1,2,3];
    let v2 = vec![4,5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    // Clone
    let unit = Unit;
    let copied_unit = unit;
    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);
    let cloned_pair = moved_pair.clone();
    drop(moved_pair);
    println!("clone: {:?}", cloned_pair);
}