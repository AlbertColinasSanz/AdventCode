use std::fs;

#[derive(Default, Debug, Copy, Clone)]
struct Casilla {
    valor: u32,
    seleccionada: bool,
}
/*
impl Default for Casilla {
    fn default() -> Casilla {
        Casilla {
            valor: 10,
            seleccionada: false,
        }
    }
}
*/
fn main() {
    println!("Hello, world!");
    let filename = "data/info.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let test: Casilla = Default::default();

    let mut tablero: [[Casilla; 5]; 5] = [[Casilla{valor: 0, seleccionada: false}; 5]; 5];


    const MAX_NUMBER: usize = 12;
    const MAX_NUMBER2: u32 = 12;

    let mut value_co2: u8;

    let mut acc_ox: i32 = 0;

    let mut possible_ox: Vec<&[u8]> = vec![];

    let mut co2: i64 = 0;

    for line in contents.split('\n') {
        //let info: Vec<&str> = line.(' ').collect();
        let cosas = line.split_whitespace();
//        println!("line {:?}", line.split_whitespace());

        for cosa in cosas {
            print!("{}-", cosa);
        }
        println!();
    }
    //println!("Acc inicial {}", acc_ox);
    
}
