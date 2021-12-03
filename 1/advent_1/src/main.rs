use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let filename = "data/info.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");


    let mut val1 = 999999;
    let mut val2 = 999999;
    let mut val3 = 999999;
    let mut val4 = 999999;
    let mut acc = 0;

    for line in contents.split('\n') {
        val1 = line.parse().unwrap();
        if val1 + val2 + val3 > val2 + val3 + val4 {
            //Descends
            acc += 1;
        }
        val4 = val3;
        val3 = val2;
        val2 = val1;
    }
    println!("With text:\n{}", acc.to_string());

}
