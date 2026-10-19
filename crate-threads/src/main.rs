use crate_threads;

fn main() {
    let numbers = vec![0,1,2,3,4,5];

    crate_threads::spawn_and_wait(move || {
        for i in numbers {
            println!("Hello {}", i);
        }
    });

    crate_threads::spawn_and_wait(|| {
        for i in 0..10 {
            println!("alternate {}", i);
        }
    });
}
