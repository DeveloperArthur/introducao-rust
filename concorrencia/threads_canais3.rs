use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//Criando Vários Produtores Clonando o Transmissor
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); //usar clone é necessario pois
    // em rust a atribuição simples move não só o valor
    // mas também sua referencia. (ownership/ownership.rs linha 47)

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}