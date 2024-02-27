//Rust não tem nulo, tem um enum Option<T>

fn main1() {
    let algum_numero = Some(5);
    let algum_texto = "um texto";
    
    /*Se usamos None em vez de Some, precisamos dizer ao Rust 
    qual é o tipo de Option<T> que nós temos, porque o compilador 
    não consegue inferir qual tipo estará contido na variante 
    Some apenas olhando para um valor None. */
    let numero_ausente: Option<i32> = None;
}

/* Somente quando temos um Option<T> vamos ter de nos preocupar com 
a possibilidade de não haver um valor, e o compilador vai se 
certificar de que nós estamos tratando este caso antes de usar o
valor.

E você tem que converter um Option<T> em um T antes de poder 
executar operações com ele. Geralmente, isso ajuda a detectar 
um dos problemas mais comuns com valores nulos: assumir que algo 
não é nulo quando, na verdade, ele é.

Só de não ter que se preocupar com a possibilidade de ter deixado 
um valor nulo escapar já lhe dá mais confiança em seu código. 
Pra ter um valor que pode ser nulo em algum momento, você precisa, 
explicitamente, marcá-lo como sendo do tipo Option<T>. 

A partir daí, sempre que for usar o valor, você será obrigado 
a tratar, de forma explícita, o caso do valor sendo nulo. 
Sempre que houver um valor que não seja um Option<T>, você pode 
assumir, com segurança, que o valor não é nulo. 

Esta foi uma decisão deliberada de projeto do Rust para limitar 
as sutilezas dos valores nulos e aumentar a segurança do código.*/

fn main() {
    let result1 = return_option(150);
    let result2 = return_option(50);

    // unwrap é usado para acessar o valor dentro de Option
    let result2 = result2.unwrap(); // <- ERRO pois result2 é None

    //o certo é verificar se há algo em option primeiro:
    if result2.is_some() {
        let result2 = result2.unwrap();
        println!("conteudo dentro de result2: {:?}", result2);
    } else {
        println!("result2 is none");
    }

    //existe tambem o método is_none:
    if result1.is_none() {
        println!("result1 is none");
    } else {
        let result1 = result1.unwrap();
        println!("conteudo dentro de result1: {:?}", result1);
    }
}

fn return_option(n: u8) -> Option<String> {
    if n > 100 {
        return Some(String::from("maior que 100"));
    } else {    
        None
    }
}