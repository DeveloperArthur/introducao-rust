use std::thread;
use std::time::Duration;
use thread::JoinHandle;

fn main1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    /* saida:

    hi number 1 from the main thread!
    hi number 1 from the spawned thread!
    hi number 2 from the main thread!
    hi number 2 from the spawned thread!
    hi number 3 from the main thread!
    hi number 3 from the spawned thread!
    hi number 4 from the main thread!
    hi number 4 from the spawned thread!
    hi number 5 from the spawned thread!

    Observe que quando o thread principal de um programa Rust é
    concluído, todos os threads gerados são encerrados, independentemente
    de terem terminado a execução ou não */
}

//-------------------------------------------------------------
//Aguardando que todos os threads terminem

fn main2() {
    let handle: JoinHandle<_> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // <- faz a thread principal esperar e não termina até que o thread gerado seja concluído.
}

//--------------------------------------------------------------
// Vamos ver o que acontece quando nos movemos
// handle.join()antes do forloop in main:

fn main() {
    let handle: JoinHandle<_> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    /* O thread principal irá esperar que o thread gerado
    termine e então executará seu forloop,
    sendo assim, a saída não fica intercalada */
}