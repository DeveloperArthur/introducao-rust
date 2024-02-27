use std::io;

fn main() {
    let (mut n, mut prox, mut inicio, mut ant, mut cont) = (String::new(), 0, 1, 0, 0);
    
    io::stdin().read_line(&mut n)
        .expect("Falha ao ler entrada");
    
    let n: u8 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Falha ao converter String para int");
        },
    };

    for i in 0..n {
        if cont == 0 {
            print!("{}", prox);
            cont += 1;
        } else if cont != n {
            ant = inicio;
            inicio = prox;
            prox = ant + inicio;
            print!(" {}", prox);
            cont += 1;
        } else if cont == n {
            print!("{}", prox);
        }
    }

    println!("");
}