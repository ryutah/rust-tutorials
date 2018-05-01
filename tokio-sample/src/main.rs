extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener
        .incoming()
        .for_each(|socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            let connection = io::write_all(socket, "hello workd\n").then(|res| {
                println!("wrote mesage; success={:?}", res.is_ok());
                Ok(())
            });

            tokio::spawn(connection);

            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });

    println!("server runninng on localhost:8080");
    tokio::run(server);
}
