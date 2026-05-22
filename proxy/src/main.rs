use std::net::{TcpListener, TcpStream}; //using this module to be able to use TCPListener, TCPStream, UDPSocket...
use std::thread;
use std::collections::HashMap;//HasMap to store all the made requisitions from the web


fn main() {

    
    let thread1 = thread::Builder::new().name("THREAD 1".to_string())
        .spawn(move || 
        {
            println!("Working on the thread one");
        });
    let thread1_join_handle = thread.expect("").join();

    let thread2 = thread::Builder::new().name("THREAD 2".to_string())
        .spawn(move ||
        {
            println!("Working on the thread two");
        });
    let thread2_join_handle = thread.expect("").join();
}
