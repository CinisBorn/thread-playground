use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let fact = Vec::from(["Elden", "Ring", "is", "very", "good"]);

        for f in fact {
            tx.send(f).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let recommendation = Vec::from([
           "Please","try", 
           "Dark","Souls", 
           "Series","too"
        ]); 
       
        for r in recommendation {
           tx1.send(r).unwrap();
           thread::sleep(Duration::from_secs(1));
        }
    });

    for f in rx {
        println!("{}", f);
    }
}
