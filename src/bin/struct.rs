#![allow(unused)]

// Struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Point3d (f32, f32, f32);

struct Empty;
#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f32,
}

fn main () {
    // Create
    let p = Point {x: 1.0, y: 2.0};// to access the fields of the struct we use dot notation p.x, p.y
    let p2 = Point {x: 3.0, y: 4.0};

    let p3 = Point3d(1.0, 2.0, 3.0);

    let c = Circle {center: p, radius: 5.0};

    // Debug put the attribute #[derive(Debug)] above the struct definition to be able to print the struct using {:?} in println! macro
    // println!("{:?}", p);
    println!("{:?}", p2);
    println!("{:?}", p3);
    println!("{:?}", c);

    // Shortcut
    let x = 1.0;
    let y = 2.0;
    let p4 = Point {x, y};// if the variable name is the same as the field name we can use this shortcut

    // Copy fields
    let p5 = Point {x: p4.x, y: p4.y};// we can also copy the fields of another struct to create a new struct
    let p6 = Point {x: p4.x, y: p4.y};// we can

    // Update
    let mut p7 = Point {x: 5.0, ..p4};// we can also update the fields of a struct using the .. syntax 
    p7.x += 1.0;// we can also update the fields of a struct using dot notation
    println!("{:?}", p7);

}