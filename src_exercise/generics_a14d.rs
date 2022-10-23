use std::marker::PhantomData;
use std::ops::Add;

#[derive(PartialEq)]
struct PhantomTuple<A,B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {first: A, phantom: PhantomData<B>}

#[derive(Debug, Clone, Copy)]
enum Inch{}
#[derive(Debug, Clone, Copy)]
enum Mm {}
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn main() {

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    println!("one foot + one_foot = {:?} in", two_feet.0);
    println!("one meter + one_meter = {:?} mm", two_meters.0);

    // CLOSNG BRACKETS
}

