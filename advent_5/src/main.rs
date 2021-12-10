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
    win: bool,
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
            print!("{}\t", t.tabla[i][j]);
        }            
        print!("\n");
    }
    print!("\n");
}

fn checkWin(t: Tablero) -> bool{
    if t.total >= 5{
        //printTablero(t);
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
            //println!("columna {}", columna);
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
            //println!("fila {}", fila);
        }
    }

    return false;
}

fn calculaGanador(t: Tablero, n: u32) {
    let mut acc = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !t.tabla[i][j].seleccionada {
                acc += t.tabla[i][j].valor;
            }
        }
    }
    println!("Final {}", acc*n);
}

fn main() {
    println!("Hello, world!");
    let filename = "data/info.txt";
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    
    const MAX_CASILLAS: usize = 1000;
    let mut tableros: [[u16; MAX_CASILLAS]; MAX_CASILLAS] = [[0; MAX_CASILLAS]; MAX_CASILLAS];

    let mut tablero = Tablero {
        tabla: [[Casilla{valor: 0, seleccionada: false}; 5]; 5], 
        total: 0, 
        win: false
    };

    let mut i = 0;
    let mut loto = "".split('p');

    for line in contents.split('\n') {
        let coord = line.split(" -> ");

        let mut x_inicio: u16 = 0;
        let mut x_final: u16 = 0;
        let mut y_inicio: u16 = 0;
        let mut y_final: u16 = 0;

        let mut inicio: bool = true;

        for c in coord{
            let axis = c.split(",");
            let mut x: bool = true;

            for a in axis {
                //println!("{:?}", a);
                if x && inicio {
                    x_inicio = a.parse().unwrap();
                }
                if !x && inicio {
                    y_inicio = a.parse().unwrap();
                }
                if x && !inicio {
                    x_final = a.parse().unwrap();
                }
                if !x && !inicio {
                    y_final = a.parse().unwrap();
                }
                
                x = false;
            }
            inicio = false;
        }
        
        println!("{},{} -> {},{}", x_inicio, y_inicio, x_final, y_final);
        let diff_x = x_inicio - x_final;
        let diff_y = y_inicio - y_final;

    }
}