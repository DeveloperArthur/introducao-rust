use std::thread;

/* O código abaixo mostra uma tentativa de criar um vetor no
thread principal e usá-lo no thread gerado.
No entanto, o código ainda não funciona */
fn main1() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn( || { //ERROR: may outlive borrowed value `v`
       println!("Here's a vector: {:?}", v);
    });

    /* o closure tenta emprestar v. No entanto, há um problema:
    Rust não consegue dizer por quanto tempo o thread gerado
    será executado, então ele não sabe se a referência a v
    será sempre válida.

    Se o Rust deixasse emprestar uma variavel da thread principal
    para outra thread, um erro poderia acontecer, por exemplo
    se eu dropasse a variavel v na thread principal: */

    // drop(v);

    /* O thread com closure tentaria capturar uma referencia de v
    e o thread principal droparia v. ou seja, v não é mais válida.

     O Rust não deixa isso acontecer para evitar problemas de
     concorrencia e segurança de acesso as referencias */

    handle.join().unwrap();
}

//--------------------------------------------------------------
//Movendo o vetor para a thread com closure com comando move

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    println!("Here's a vector: {:?}", v); //ERROR value borrowed here after move

    /* Ao dizer ao Rust para mover a propriedade v para o thread
    gerado estamos garantindo ao Rust que o thread principal não
    será v mais usado. */

    handle.join().unwrap();
}