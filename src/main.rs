// use std::fs::File;
// use std::io::Read;
use std::fs::read_to_string;

// fn read_file_1(path: &str) -> std::string::String {
//     let mut file = File::open(path).expect("File not found"); 
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).expect("Unable to read the file");

//     return contents;
// }

fn read_file(path: &str) -> String {
   let data = match read_to_string(path) {
        Err(err) => panic!("Couldn't read: {}", err),
        Ok(data) => data,
   };
   return data;
}

fn main() {
    let hello = read_file("hello.txt");
    let world = read_file("rust.txt");

    println!("Content is: {} and {}", hello, world);
}