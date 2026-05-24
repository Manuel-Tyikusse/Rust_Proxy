use std::net::{TcpListener, TcpStream, ToSocketAddrs}; //using this module to be able to use TCPListener, TCPStream, UDPSocket...
use std::thread;
use std::collections::HashMap;//HasMap to store all the made requisitions from the web
use std::io::{Read, Write};
use std::io::Result;
use std::sync::Arc;

fn handle_client(mut stream: TcpStream)->Result<()>{
    let mut buf = [0; 1024];
    loop{
        let buf_read = stream.read(&mut buf)?;
        if buf_read ==  0 {
            return Ok(());
        }
        stream.write_all(&buf[..buf_read])?;
    }
}


fn main()-> std::io::Result<()>{

    let port_to_read = "127.0.0.1:8080"; //IP address and the port ip{127.0.0.1}, port{8080}
    
    let mut cached_wesites = Arc::new(HashMap::<String, String>::new());
    let listener = TcpListener::bind(port_to_read)?;

    let thread1 = thread::Builder::new().name("Thread 1".to_string())
        .spawn(move || {
            for new_web in port_to_read.incoming()?{
                handle_client(new_web?)?;
                println!("Currenty thread {thread::current().name():?}");
                let web_site_url = new_web.to_sock_addrs();
                match web_site_url{//Objectivo> verificar se o endereço é puro, adicionar ou remover o http:// ou remover e adicionar a porta
                    //Mostrar o IP do site, seu url, e a tempo do inicio de conexão
                    
                }
                println!()
                //antes de abrir o site deve verificar se o site está dentro do cache , caso contrario adicionar ele
            }
        });
    let thread1_join = thread1.expect("Complete").join();

    let thread2 = thread::Builder::new().name("Thread 2".to_string())
        .spawn(move || {

        });
    let thread2_join = thread2.expect("Complete").join();

    let thread3 = thread::Builder::new().name("Thread 3".to_string())
        .spawn(move || {

        });
    let thread3_join = thread3.expect("Complete").join();

    let thread4 = thread::Builder::new().name("Thread 4".to_string())
        .spawn(move || {

        });
    let thread4_join = thread4.expect("Complete").join();

    let thread5 = thread::Builder::new().name("Thread 5".to_string())
        .spawn(move || {

        });
    let thread5_join = thread5.expect("Complete").join();


    let thread6 = thread::Builder::new().name("Thread 6".to_string())
        .spawn(move || {

        });
    let thread6_join = thread6.expect("Complete").join();

    let thread7 = thread::Builder::new().name("Thread 7".to_string())
        .spawn(move || {

        });
    let thread7_join = thread7.expect("Complete").join();


    let thread8 = thread::Builder::new().name("Thread 8".to_string())
        .spawn(move || {

        });
    let thread8_join = thread8.expect("Complete").join();


    let thread9 = thread::Builder::new().name("Thread 9".to_string())
        .spawn(move || {

        });
    let thread9_join = thread9.expect("Complete").join();

    let thread10 = thread::Builder::new().name("Thread 10".to_string())
        .spawn(move || {

        });
    let thread10_join = thread10.expect("Complete").join();

    
    Ok(())

}
