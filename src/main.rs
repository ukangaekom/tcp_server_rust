// Importing necessary modules from rust libraries
use std::io{Read, Write};
use std::net{TcpListener, TcpStream};


// Defining funcion to handle client

fn handle_client(mut stream: TcpStream){

    // This function will handle multiple connections on port 8080


    // This is a buffer to read data from client

    let mut buffer = [0; 1024];

    // This line reads data from the stream to store in the buffer
    stream.read(&mut buffer).expect("Failed to read from client");

    // Converting the data in the buffer to a utf-8 encoded string
    let requst = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    let response = "Hello, Client!".as_bytes();

    // Responding to client
    stream.write(response]).expect("Failed to write response");
    
}


// Entry point
fn main(){
    // declaring a listener to bind tcp to local host
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listenerincoming(){

        match stream {

            Ok(stram) =>{
                std::thread::spawn(|| handle_client(stream));
            }

        }

    }

}