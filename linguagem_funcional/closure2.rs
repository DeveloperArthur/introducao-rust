use std::time::Duration;
use std::thread;

fn main1() {
    /* Define um closure que recebe um parâmetro num
    do tipo u32 e retorna um valor do mesmo tipo */
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let result = expensive_closure(42);
    println!("Resultado: {}", result);
}

/* função que adiciona 1 ao seu parâmetro e
closures que tem o mesmo comportamento, para comparação: */
fn add_one_v1(x: u32) -> u32 {
    x + 1
}

fn main() {
    let add_one_v2 = |x: u32| -> u32 {
        x + 1
    };

    let add_one_v3 = |x| {
        x + 1
    };

    let add_one_v4 = |x| x + 1;

    let r1 = add_one_v1(1);
    let r2 = add_one_v2(1);
    let r3 = add_one_v3(1);
    let r4 = add_one_v4(1);

    println!("{} {} {} {}", r1, r2, r3, r4);

    let r5 = add_one_v2(5.6);
    // ERROR: como passamos inteiro na linha 34,
    // o rust infere o parametro como tipo inteiro para a closure
    // se tentarmos passar qualquer outro tipo resultará em erro
}



