use delta_command_api::*;

// this eample will use websocket+json

use std::net::TcpListener;
use std::thread::spawn;

use tungstenite::accept_hdr;
use tungstenite::handshake::server::Request;

use tungstenite::Message::Text;

fn main() {
    let server = TcpListener::bind("127.0.0.1:29031").unwrap();
    println!("server running on {:?}", server.local_addr());
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.path);
                // println!("The request's headers are:");
                // for &(ref header, _ /* value */) in req.headers.iter() {
                //     println!("* {}", header);
                // }

                // Let's add an additional header to our response to the client.
                let extra_headers = vec![(String::from("MyCustomHeader"), String::from(":)"))];
                Ok(Some(extra_headers))
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            loop {
                let msg = websocket.read_message().unwrap();
                if
                /* msg.is_binary() || */
                msg.is_text() {
                    println!(":{:?}:", msg);
                    let answer = Text(run_json(msg.to_text().unwrap()));
                    websocket.write_message(answer).unwrap();
                }
            }
        });
    }
}
