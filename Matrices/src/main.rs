const NF: usize = 120;
const NC: usize = 120;

struct Matriz {
    nro_filas:    usize,
    nro_columnas: usize,
    celdas: [[u64; NC]; NF],
}

impl Matriz {
    fn new(nro_filas: usize, nro_columnas: usize) -> Matriz {
        Matriz {
            nro_filas,
            nro_columnas,
            celdas: [[0; NC]; NF],
        }
    }

    fn obt_nro_filas(&self) -> usize {
        self.nro_filas
    }

    fn obt_nro_columnas(&self) -> usize {
        self.nro_columnas
    }

    fn get_celda(&self, f: usize, c: usize) -> u64 {
        self.celdas[f][c]
    }

    fn set_celda(&mut self, f: usize, c: usize, valor: u64) {
        self.celdas[f][c] = valor;
    }

    fn sumar_elementos(&self) -> u64 {
        if self.nro_filas > 0 && self.nro_columnas > 0 {
            let mut suma = 0;
            for f in 0..self.nro_filas {
                for c in 0..self.nro_columnas {
                    suma += self.celdas[f][c]
                }
            }
            return suma
        }
        0
    }

    fn pares(&self) -> u64 {
        if self.nro_filas > 0 && self.nro_columnas > 0 {
            let mut pares = 0;
            for f in 0..self.nro_filas {
                for c in 0..self.nro_columnas {
                    if self.celdas[f][c] % 2 == 0 {
                        pares += 1
                    }
                }
            }
            return pares
        }
        0
    }

    fn maximo(&self) -> u64 {
        if self.nro_filas > 0 && self.nro_columnas > 0 {
            let mut mayor = self.celdas[0][0];
            for f in 0..self.nro_filas {
                for c in 0..self.nro_columnas {
                    if self.celdas[f][c] > mayor {
                        mayor = self.celdas[f][c]
                    }
                }
            }
            return mayor
        }
        0
    }

    fn promedio(&self) -> u64 {
        if self.nro_filas > 0 && self.nro_columnas > 0 {
            return self.sumar_elementos() / (self.nro_columnas as u64 * self.nro_filas as u64)
        }
        0
    }

    fn suma_fila(&self, f: usize) -> u64 {
        if self.nro_filas > 0 && self.nro_columnas > 0 && f > 0 && f <= self.nro_filas {
            let mut suma = 0;
            for c in 0..self.nro_columnas {
                suma += self.celdas[f-1][c];
            }
            return suma
        }
        0
    }

    fn suma_columna(&self, c: usize) -> u64 {
        if self.nro_filas > 0 && self.nro_columnas > 0 && c > 0 && c <= self.nro_columnas {
            let mut suma = 0;
            for f in 0..self.nro_filas {
                suma += self.celdas[f][c-1];
            }
            return suma
        }
        0
    }

    fn recorrido_diagonal_ascendente(&self) {
        for fila in (0..self.nro_filas).rev() {
            let mut f = fila;
            let mut c = 0;
            while f < self.nro_filas && c < self.nro_columnas {
                print!("{} ", self.celdas[f][c]);
                f += 1;
                c += 1
            }
        }

        for col in 1..self.nro_columnas {
            let mut c = col;
            let mut f = 0;
            while f < self.nro_filas && c < self.nro_columnas {
                print!("{} ", self.celdas[f][c]);
                f += 1;
                c += 1;
            }
        }

        println!();
    }

    fn recorrido_en_x(&self) {
        let limite = self.nro_filas/2;
        for fila in 0..limite {
            let mut f = fila;
            for c in 0..self.nro_columnas {
                print!("{} ", self.celdas[f][c]);
                f += 1;
            }
            f = fila;
            for c in (0..self.nro_columnas).rev() {
                print!("{} ", self.celdas[f][c]);
                f += 1;
            }


        }

        println!();
    }

    fn recorrido_filas_ida_vuelta(&self) {
        for f in 0..self.nro_filas {
            if f % 2 == 0 {
                for c in 0..self.nro_columnas {
                    print!("{} ", self.celdas[f][c])
                }
            }else{
                for c in (0..self.nro_columnas).rev() {
                    print!("{} ", self.celdas[f][c])
                }
            }
        }

        println!();
    }

    fn recorrido_col_ida_vuelta(&self) {
        for c in 0..self.nro_columnas {
            if c % 2 == 0 {
                for f in 0..self.nro_filas {
                    print!("{} ", self.celdas[f][c]);
                }
            }else{
                for f in (0..self.nro_filas).rev() {
                    print!("{} ", self.celdas[f][c]);
                }
            }
        }

        println!();
    }

    fn recorrido_espiral(&self) {
        let mut limite_sup = 0;
        let mut limite_der = self.nro_columnas - 1;
        let mut limite_inf = self.nro_filas - 1;
        let mut limite_izq = 0;
        while limite_der >= limite_izq && limite_inf >= limite_sup {
            for c in limite_izq..=limite_der {
                print!("{} ", self.celdas[limite_sup][c]);
            }
            limite_sup += 1;
            print!("  ");

            for f in limite_sup..=limite_inf {
                print!("{} ", self.celdas[f][limite_der])
            }

            if limite_der == 0 { 
                break; 
            }
            limite_der -= 1;
            print!("  ");

            if limite_sup <= limite_inf {
                for c in (limite_izq..=limite_der).rev() {
                    print!("{} ", self.celdas[limite_inf][c])
                }
                limite_inf -= 1;
                print!("  ");
            }

            if limite_izq <= limite_der {
                for f in ((limite_sup)..=(limite_inf)).rev() {
                    print!("{} ", self.celdas[f][limite_izq])
                }
                limite_izq += 1;
                print!("  ");
            }

        }

        println!()
    }

    fn sumar_fila_columna(&self, f: usize, c: usize) -> u64 {
        self.suma_fila(f) + self.suma_columna(c)
    }

    fn sumar_diagonal_principal(&self) -> u64 {
        if self.nro_columnas == self.nro_filas {
            let mut suma = 0;
            for i in 0..self.nro_columnas{
                suma += self.celdas[i][i];
            }
            return suma;
        }else{
            println!("  La matriz no es cuadrada");
            0
        }
        
    }

    fn sumar_diagonal_secundaria(&self) -> u64 {
        if self.nro_columnas == self.nro_filas {
            let mut suma = 0;
            for i in 0..self.nro_columnas{
                suma += self.celdas[i][(self.nro_columnas - 1 ) - i];
            }
            return suma;
        }else{
            println!("  La matriz no es cuadrada");
            0
        }
    }

    fn suma_l(&self, f: usize) -> u64 {
        let mut suma = 0;
        if f <= self.nro_filas {
            for c in 0..self.nro_columnas {
                suma += self.celdas[f - 1][c];
            }
            for i in (f)..self.nro_filas {
                suma += self.celdas[i][self.nro_columnas - 1]
            }
        }
        suma
    }

    // ── Mostrar ───────────────────────────────────────────────────

    fn mostrar(&self) {
        println!();
        for f in 0..self.nro_filas {
            print!("  ");
            for _ in 0..self.nro_columnas {
                print!("┌─────────┐");
            }
            println!();
            print!("  ");
            for c in 0..self.nro_columnas {
                print!("│{:^9}│", self.celdas[f][c]);
            }
            println!();
            print!("  ");
            for _ in 0..self.nro_columnas {
                print!("└─────────┘");
            }
            println!();
        }
        print!("  ");
        for c in 0..self.nro_columnas {
            print!("{:^11}", format!("c{}", c));
        }
        println!("\n");
    }

    // ── Operaciones básicas ───────────────────────────────────────

}


// ═══════════════════════════════════════════════════════════════════
//  MAIN
// ═══════════════════════════════════════════════════════════════════

fn main() {
    println!("════════════════════════════════════════");
    println!("  Matrices - POO — Programación I      ");
    println!("════════════════════════════════════════");

    // ── crear la matriz 3x3 ──────────────────────────────────────
    let mut m = Matriz::new(4, 4);

    // ── cargar datos directamente ────────────────────────────────
    //        fila  col  valor
    m.celdas[0][0] = 1;   m.celdas[0][1] = 2; m.celdas[0][2] = 3;   m.celdas[0][3] = 4;
    m.celdas[1][0] = 5;   m.celdas[1][1] = 6; m.celdas[1][2] = 7;   m.celdas[1][3] = 8;
    m.celdas[2][0] = 9;   m.celdas[2][1] = 10; m.celdas[2][2] = 11;   m.celdas[2][3] = 12;
    m.celdas[3][0] = 13;   m.celdas[3][1] = 14; m.celdas[3][2] = 15;   m.celdas[3][3] = 16;
    

    // ── mostrar ──────────────────────────────────────────────────
    println!("Matriz original:");
    m.mostrar();

    println!("  La suma de los elementos de la matriz es: {}", m.sumar_elementos());
    println!("  La cantidad de numeros pares de la matriz es: {}", m.pares());
    println!("  El elemento mayor de la matriz es: {}", m.maximo());
    println!("  El promedio de los elementos de la matriz es: {}", m.promedio());
    println!("  La suma de la fila de la matriz es: {}", m.suma_fila(1));
    println!("  La suma de la columna de la matriz es: {}", m.suma_columna(3));

    println!();
    m.recorrido_diagonal_ascendente();
    m.recorrido_en_x();
    m.recorrido_filas_ida_vuelta();
    m.recorrido_col_ida_vuelta();
    m.recorrido_espiral();
    println!();
    println!("  La suma de la fila y de la columna es: {}", m.sumar_fila_columna(2, 3));
    println!("  La suma de la diagonal principal es: {}", m.sumar_diagonal_principal());
    println!("  La suma de la diagonal secundaria es: {}", m.sumar_diagonal_secundaria());
    println!("  La suma en L es: {}", m.suma_l(2));

}