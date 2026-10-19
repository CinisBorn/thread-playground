use std::sync::{Mutex, Arc};

fn main() {
    let state = Arc::new(Mutex::new(0));
    let mut handlers = Vec::new();

    for _ in 0..10 {
        let state = Arc::clone(&state);
        
        let handle = std::thread::spawn(move || {
            let mut num = state.lock().unwrap();

            *num += 1;
        });
        
        handlers.push(handle);
    }

    for handle in handlers {
         handle.join().unwrap();
    }

    println!("{}", state.lock().unwrap());
}
