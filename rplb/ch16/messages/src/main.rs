use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let handle1 = thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // tx.send(val).unwrap();
    });

    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("this"),
            String::from("side"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // tx.send(val).unwrap();
    });

    // let received = rx.recv().unwrap();
    for received in rx {
        println!("Got: {}", received);
    }
    handle1.join().unwrap();
    handle2.join().unwrap();
}
