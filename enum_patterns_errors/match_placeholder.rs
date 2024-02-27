// match with enum
#[derive(Debug)]
enum Estado {
    Alabama,
    Alaska,
}

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter(Estado),
}

fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => 1,
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter(estado) => {
            println!("Quarter do estado {:?}!", estado);
            25
        }
    }
}

fn main1() {
    let moeda = Moeda::Quarter(Estado::Alaska);

    let moeda = valor_em_cents(moeda);

    println!("{}", moeda);
}
//------------------------------------------------------------------------
// match with option
fn mais_um(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main2() {
    let cinco = Some(5);
    let seis = mais_um(cinco);
    let nenhum = mais_um(None);
}
//------------------------------------------------------------------------
// placeholder
fn main() {
    let algum_valor_u8 = 0u8;

    match algum_valor_u8 {
        1 => println!("um"),
        3 => println!("três"),
        5 => println!("cinco"),
        7 => println!("sete"),
        _ => (),
    }
    
    /*O padrão _ casa com qualquer valor. Colocando ele depois dos 
    demais braços, o _ vai casar com todos os casos possíveis que 
    não foram especificados antes dele. */
}