extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivinhe o número!");
    let mut tentativas = 0;
    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Digite o seu palpite.");

        //em rust as variaveis sao imutaveis por padrao
        //let mut = variavel mutável / apenas let = variável imutável
        let mut palpite = String::new();
    
        //assim como as variáveis, referências são imutáveis por padrão
        // Por isso, precisamos escrever &mut palpite, em vez de apenas &palpite
        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");
    
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Você disse: {}", palpite);
    
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => {
                tentativas += 1;
                println!("Muito baixo!");
            }
            Ordering::Greater => {
                tentativas += 1;
                println!("Muito alto!")
            }
            Ordering::Equal => {
                println!("Você acertou!");
                println!("Foram no total {} tentativas", tentativas);
                break;
            }
        }
    }
}
