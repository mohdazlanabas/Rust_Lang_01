use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<A,B>(PhantomData<B>);
#[derive(PartialEq)]
struct PhantomStruct<A, B> {first: A, phantom: PhantomData<B> }


struct A;
struct S(A);
struct SGen<T>(T);
struct Single(A);
struct SingleGen<T>(T);

fn reg_fn(_s: S){}
fn gen_spec_t(_s: SGen<A>){}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generic<T>(_s: SGen<T>) {}

fn main() {
    let _s = Single(A);
    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
    let _tuple1:PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2:PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic::<char>(SGen('a'));
    generic(SGen('c'));

}