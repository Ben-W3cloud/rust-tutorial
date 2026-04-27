#[allow(unused)]

fn main () {
    //String = vector of u8 valid UTF-8 and 
    //&str =  slice of u8 are the two ways to handle strings in rust
    //String is used when you want to mutate or data needs to be owned but &str is for when it is read only
    //String
    let message = String::from(" Hello Rust");
    let len = message.len();

    //string slice - &str, you can hardcode it or refernce a string as a string slice 
    let msg = String::from(" Hello Rust abnd Cargo");
    let s: &str = &msg[..];
    let len = s.len();

    //declrig with string lietral( stored inside binary, immutable, slice pointing to a specific part of the binary)
    let hello = " Hello Rust";
    let s = r#"
        {
        {
        {
        {
    "#;

    //deref coercion
    // it converting a slice of a string to ref of str, convenient when dealing with functions
    let msg = String::from(" Hello Rust abnd Cargo");
    let s: &str = &msg[..];

    //mutating string'
    let mut msge: String = "Hello Rust".to_string();
    msge += "!";

    let lang = "Rust";
    let char = "C";
    let msg = format!("Hello {lang} {char}");


}