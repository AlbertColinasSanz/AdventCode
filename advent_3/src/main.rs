use std::fs;

fn main() {
    println!("Hello, world!");
    let filename = "data/info.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in contents.split('\n') {
        let info: Vec<&str> = line.split(' ').collect();
        let op = info[0];
        let value: u32 = info[1].parse().unwrap();
        match op {
            "forward" => {
                forward += value; 
                depth += value*aim;
            },
            "down" => aim += value,
            "up" => aim -= value,
            _ => println!("Input does not equal any value"),
        }
    }
    println!("{}", depth*forward);
}
