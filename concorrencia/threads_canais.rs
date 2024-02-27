use std::sync::mpsc;
use std::thread;

//Usando passagem de mensagens para transferir dados entre threads
fn main() {
    /* Para realizar a simultaneidade no envio de mensagens,
    a biblioteca padrão do Rust fornece uma implementação de canais.
    Um canal é um conceito geral de programação pelo qual os dados
    são enviados de um thread para outro. */

    //Criamos um novo canal
    let (tx, rx) = mpsc::channel();
    /* mpsc é um acronimo para "produtor múltiplo, consumidor único" */

    //Criamos uma thread com closure, movendo tx
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); //transmissor envia mensagem
    });

    //receptor recebe a mensagem
    let received = rx.recv().unwrap();
    /* O receptor possui dois métodos úteis: recve try_recv

    recv, abreviação de receiver , que bloqueará a execução do
    thread principal e aguardará até que um valor seja enviado pelo canal.
    Assim que um valor for enviado, recvele será retornado em um arquivo
    Result<T, E>. Quando o transmissor fechar, recv retornará um erro para
    sinalizar que não haverá mais valores chegando.

    try_recvmétodo não bloqueia, mas retornará imediatamente Result<T, E>
    um Ok valor contendo uma mensagem se houver alguma disponível e um
    Err valor se não houver nenhuma mensagem desta vez. */

    println!("Got: {}", received);
}