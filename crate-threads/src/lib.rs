pub fn spawn_and_wait<F, T>(handle: F) 
where 
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static
{
    let handle = std::thread::spawn(handle);
    handle.join().unwrap();
}