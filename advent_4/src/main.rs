use std::fs;

#[derive(Default, Debug, Copy, Clone)]
struct Casilla {
    valor: u32,
    seleccionada: bool,
}
impl std::fmt::Display for Casilla {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.seleccionada == true {
            return write!(f, "t{}", self.valor);
        } else {
            return write!(f, "f{}", self.valor);
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
struct Tablero {
    tabla: [[Casilla; 5]; 5],
    total: u8,
}

impl std::fmt::Display for Tablero {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.debug_list().entries(self.tabla.iter()).finish();
    }
}

fn printTablero(t: Tablero){
    let mut acc: String = "".to_owned();
    for i in 0..5 {
        for j in 0..5 {
            acc.push_str(write!(f, "{}", self.tabla[i][j]));
        }
        acc.push_str(write!(f, "\n"));
    }
    write!(f, "(value a: {}, value b: {})", self.a, self.b);
    return acc;

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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn checkWin(t: Tablero) -> bool{
    if t.total >= 5{
        for i in 0..5 {
            let mut columna = 0;
            for j in 0..5 {
                if t.tabla[i][j].seleccionada {
                    columna += 1;
                } else {
                    break;
                }
            }
            if columna == 5{
                return true;
            }
        }
        for i in 0..5 {
            let mut fila = 0;
            for j in 0..5 {
                if t.tabla[j][i].seleccionada {
                    fila += 1;
                } else {
                    break;
                }
            }
            if fila == 5{
                return true;
            }
        }
    }

    return false;
}

fn main() {
    println!("Hello, world!");
    let filename = "data/info.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let test: Casilla = Default::default();

    let mut tableros: Vec<Tablero> = Vec::new();
    
    //let mut tablero = [[Casilla{valor: 0, seleccionada: false}; 5]; 5];
    let mut tablero = Tablero{tabla: [[Casilla{valor: 0, seleccionada: false}; 5]; 5], total: 0};

    let lineas = contents.split('\n');
    print_type_of(&lineas);

    let mut i = 0;
    let mut loto = "".split('p');

    for line in contents.split('\n') {
        let cosas = line.split_whitespace();

        if line.len() > 30 {
            // primera linea
            loto = line.split(',');
            
        } else if line.len() > 0 {
            // Tableros
            let mut j = 0;
            for cosa in cosas {
                //print!("{}-", cosa, );
                tablero.tabla[i][j] = Casilla{valor: cosa.parse().unwrap(), seleccionada: false};
                j += 1;
            }
            i += 1;
            if i == 5{
                tableros.push(tablero.clone());
                tablero = Tablero{tabla: [[Casilla{valor: 0, seleccionada: false}; 5]; 5], total: 0};
            }
        } else {
            i = 0;
        }

    }
    //println!("Inicial {:?}", tableros);
    println!("");
    //println!("Inicial {:?}", loto);

    for n in loto {
        let number: u32 = n.parse().unwrap();

        'outer: for t in &mut tableros {
            for i in 0..5 {
                for j in 0..5 {
                  if t.tabla[i][j].valor == number{
                    t.tabla[i][j].seleccionada = true;
                    t.total += 1;
                    if checkWin(*t){
                        println!("Ganador - {} - {}", t, number);
                        break 'outer;
                    }
                  }  
                }
            }
        }
    }
    
}
