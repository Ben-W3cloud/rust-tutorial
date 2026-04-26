#![allow(unused)]
#[derive(Debug)]
struct Lang {
    language:String,
    version: String
}

fn main() {
    let lang = Lang {
      language: "Rust".to_string(),
      version: "1.23".to_string(),  
    };
    // {:?} => used to print strcuts or lists but {:#?} => prints with line breaks
    println!("Hello, world!. I am officially learning {:?} ", lang);
    
    println!("Hello, world!. I am officially learning {:#?} ", lang);

}
