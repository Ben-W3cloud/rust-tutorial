#![allow(unused)]
// attributes for enums
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    RGBA(u8,u8,u8,f32),//enum that takes in params
    Hex(String),
    Hsl{h: u8, s: u8, l: u8},
}

fn main(){
    // Enum
    let color1: Color = Color::Red;
    let color2: Color = Color::Green;
    let color3: Color = Color::RGBA(0, 0, 255, 0.1);
    let color4: Color = Color::Hex("#fffffff". to_string());
    let color5: Color = Color::Hsl{h:12, s:52 , l: 200};

    // Attributes - Debug
    println!("{:?} {:?} {:?} {:?} {:?} ", color1, color2, color3, color4,  color5);

    // Partialeq - compare enums you use attribute partialeq
    println!(" {:?} ", Color::Red == Color::Green);

    //Option enum that can take 2 val = Some(11) | None , useful when fetching data from an array
    let x:Option<i32> = None;
    let y: Option<i32> =Some(11);

    //Result reps if sth was successful it retruns Ok(result) | Err("error message")
    let res: Result<u32, String> = Ok(5);
    let res: Result<u32, String> = Err(" This is a  very long Error message". to_string());

}