use std::sync::mpsc;
use std::thread;
use std::time::Duration;

//Enviando vários valores e vendo o receptor esperando
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
          String::from("hi"),
          String::from("from"),
          String::from("the"),
          String::from("thread"),
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