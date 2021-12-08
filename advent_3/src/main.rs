use std::fs;

fn main() {
    println!("Hello, world!");
    let filename = "data/info.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    const MAX_NUMBER: usize = 12;
    const MAX_NUMBER2: u32  = 12;
    let pow2: i64 = 2;

    let mut value_ox: u8;
    let mut value_co2: u8;

    let mut acc_ox: i32 = 0;
    let mut acc_co2: i32;

    let mut possible_ox: Vec<&[u8]> = vec![];
    let mut possible_co2: Vec<&[u8]> = vec![];

    let mut ox: i64 = 0;
    let mut co2: i64 = 0;

    for line in contents.split('\n') {
        //let info: Vec<&str> = line.(' ').collect();
        //println!("line {:?}", line);

        if line.chars().nth(0) == Some('0') {
            acc_ox -= 1;
        } else {
            acc_ox += 1;
        }
        possible_ox.push(line.as_bytes());
        possible_co2.push(line.as_bytes());
    }
    //println!("Acc inicial {}", acc_ox);

    if acc_ox >= 0 {
        value_ox = b'1';
        value_co2 = b'0';
    } else {
        value_ox = b'0';
        value_co2 = b'1';
    }

    let mut done_ox: i8 = 0;
    let mut done_co2: i8 = 0;

    println!("Value inicial {}", value_ox);

    for i in 0..MAX_NUMBER {
        let mut new_possible_ox: Vec<&[u8]> = vec![];
        let mut new_possible_co2: Vec<&[u8]> = vec![];

        if done_ox == 0 {
            for poss in possible_ox.iter() {
                if poss[i] == value_ox {
                    new_possible_ox.push(*poss);
                }
                // println!("> {:?}", poss);
            }
            possible_ox = new_possible_ox.clone();
        }

        if done_co2 == 0 {
            for poss in possible_co2.iter() {
                if poss[i] == value_co2 {
                    new_possible_co2.push(*poss);
                }
                // println!("> {:?}", poss);
            }
            possible_co2 = new_possible_co2.clone();
        }


        //println!("Old {:?}", possible);
        //println!("New {:?}", new_possible);

        if possible_ox.len() == 1 {
            done_ox = 1;
        }
        if possible_co2.len() == 1 {
            done_co2 = 1;
        }
        if done_co2 == 1 && done_ox == 1 {
            break;
        }
        
        if done_ox == 0 {
            acc_ox = 0;
            for p in new_possible_ox.iter() {
                if p[i + 1] == b'0' {
                    acc_ox -= 1;
                } else {
                    acc_ox += 1;
                }
            }

            if acc_ox >= 0 {
                value_ox = b'1';
            } else {
                value_ox = b'0';
            }
            println!("Value OX {:?}", value_ox);

        }

        if done_co2 == 0 {
            acc_co2 = 0;
            for p in new_possible_co2.iter() {
                if p[i + 1] == b'0' {
                    acc_co2 -= 1;
                } else {
                    acc_co2 += 1;
                }
            }
            if acc_co2 >= 0 {
                value_co2 = b'0';
            } else {
                value_co2 = b'1';
            }
            //println!("Old OX {:?}", possibleOX);
            //println!("i {:?}", i + 1);
            //println!("acc OX {:?}", accOX);
            //println!("Value OX {:?}", valueOX);
        }
    }

    println!("OX {:?}", possible_ox);
    println!("CO2 {:?}", possible_co2);

    let mut index: u32 = 1;
    for i in 0..MAX_NUMBER {
        if possible_ox[0][i] == b'1' {
            ox += pow2.pow(MAX_NUMBER2 - index);
        }
        index += 1;
    }

    index = 1;
    for i in 0..MAX_NUMBER {
        if possible_co2[0][i] == b'1' {
            co2 += pow2.pow(MAX_NUMBER2 - index);
        }
        index += 1;
    }
    println!("OX {:?}", ox);
    println!("CO2 {:?}", co2);
    println!("{:?}", co2*ox);
}
