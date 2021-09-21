use std::fs::File;

fn main() {
    //    let v = vec![0, 1, 2, 3];
    //    println!("{}", v[6]);

    let fruits = vec!["banana", "apple", "cocconut"];

    let first = fruits.get(0);

    println!("{:?}", first);

    let third = fruits.get(2);

    println!("{:?}", third);

    let fifth = fruits.get(4);

    println!("{:?}", fifth);

    //    let f = File::open("hello.txt");

    //   let f = match f {
    //      Ok(file) => file,
    //     Err(error) => panic!("Can't open the file: {:?}", error),

    //let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
