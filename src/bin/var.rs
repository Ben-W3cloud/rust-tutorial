#![allow(unused)]

fn main (){
    //variables in Rust are immutable by default
    let a: i32 =  -123;
    // a += 1 will not compile,  adding mut makes it mutable
    let mut y: i32 = 123;
    y += 1;

    let w: i32= 123;
    // to make a variable constant

    const NUM: u32 = 1;
     // you can also redeclare a variable  
    let a: i32 = 1;
    // _ is a type placeholder

    let v: Vec<_> = vec![1,2,3];

    //Scalar types
    // integers i(n)/u(n) -(2**(n-1)) ~ 2**(n -1) - 1 
    let i0: i8 = 0;
    let i1: i64 = 2309504;

    let u0: u32 = 232;
    let u1: u32 = 22;
    // i - signed{+/-}, u is unsigned{-}

    // Floats - floating numbers
    let f0: f32 = 0.0029;
    let f1: f64 = 0.09;

    //chars
    let char = 'c';

    //bool
    let b= true;

    //Type conversion
    let i: i32 = 1;

    let u: u32 = i as u32;//useful when adding signed to unsigned
    let x: u32 = u + (i as u32);

    // Min and Max
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("{}, {} ", min_i, max_i);

    // Compound Data types - tuple,
    let t:(bool,char,u32)= (false, 'c',1);

    //destructure
    let (a,b,c) = t;
    println!("{}, {} {}", a,b,c);

    let (_, b,_) = t;
    //empty
    let t = ();
    let nested = ((1,2), (3,4));    
    println!("{}, {}", nested.0.0, nested.1.0);
    //nested.0 - first 1, 0.0 => first element in first tuple

}