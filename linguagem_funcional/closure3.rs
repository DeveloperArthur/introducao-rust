use std::thread;

/* Os closures podem capturar valores de seu ambiente de três
maneiras, que mapeiam diretamente as três maneiras pelas quais uma
função pode assumir um parâmetro: emprestando de forma imutável,
emprestando de forma mutável e assumindo propriedade.
O closure decidirá qual deles usar com base no que o corpo da função
faz com os valores capturados. */

/* closure que captura uma referência imutável ao vetor nomeado list
porque ele só precisa de uma referência imutável para imprimir o valor */
fn main1() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

/* alteramos o corpo do closure para que ele adicione um elemento
ao list vetor.
O closure agora captura uma referência mutável: */
fn main2() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    //println!("Before calling closure: {:?}", list); ERROR: cannot borrow `list` as immutable because it is also borrowed as mutable

    /* o rust está reclamando porque o closure já está usando
    uma referencia imutavel de list, e o println! tenta imprimir
    a lista de forma imutavel utiliza uma porque é a menor quantidade
    de acesso list necessária para imprimi-lo, ai conflita,
    nao pode ter uma referencia mutavel e imutavel sendo usada ao mesmo tempo.
    por isso só podemos usar list depois que o empréstimo terminar
    ou seja, depois de chamar borrows_mutably()
    rust faz isso para para evitar problemas de mutabilidade e garantir
    a segurança do acesso aos dados*/

    borrows_mutably();
    println!("After calling closure: {:?}", list); //FUNCIONA
}

/* closure assumindo propriedade com a palavra chave move */
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || {
        move || println!("From closure: {:?}", list);
    };

    only_borrows();
    // println!("After calling closure: {:?}", list); ERROR: value borrowed here after move
    //ou seja, list pertence a closure, e saiu de escopo no fim do closure
}