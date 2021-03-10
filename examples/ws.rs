// This example is to try out / play with the basic websocket logic part of the project
use async_channel::{unbounded, TryRecvError};
use async_lock::Mutex;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};
use futures::future::{select, Either};
use futures::prelude::*;
use log::*;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // let's say the intervalls are events
    let mut interval = async_std::stream::interval(Duration::from_millis(1000));

    // Echo incoming WebSocket messages and send a message periodically every second.
    let (s, r) = unbounded::<Message>();
    let send_to_channel: Arc<Mutex<async_std::channel::Sender<Message>>> = Arc::new(Mutex::new(s));
    let stc = send_to_channel.clone();
    task::spawn(async move {
        loop {
            interval.next().await;
            info!("tick event");
            match stc
                .lock()
                .await
                .send(Message::Text("[event] tick".to_owned()))
                .await
            {
                Ok(_) => {}
                Err(err) => error!("Error sending event {:?}", err),
            };
        }
    });
    let mut receive_next_command = ws_receiver.next();
    let mut send_outgoing = r.recv();
    let mut remaining_join_handles = Vec::new();
    loop {
        match select(receive_next_command, send_outgoing).await {
            Either::Left((msg, send_outgoing_continue)) => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;

                        let stc = send_to_channel.clone();

                        if msg.is_text() || msg.is_binary() {
                            remaining_join_handles.push(task::spawn(async move {
                                task::sleep(Duration::from_secs(5)).await;
                                match stc.lock().await.send(msg).await {
                                    Ok(_) => {}
                                    Err(err) => error!("{:?}", err),
                                };
                            }));
                        } else if msg.is_close() {
                            break;
                        }
                        send_outgoing = send_outgoing_continue; // Continue waiting for pending outgoing messages
                        receive_next_command = ws_receiver.next(); // Receive next WebSocket message.
                    }
                    None => break, // WebSocket stream terminated.
                };
            }
            Either::Right((msg, msg_fut_continue)) => {
                if let Ok(message) = msg {
                    ws_sender.send(message).await?;
                } else {
                    error!("For some reason message wasn't able to be recieved from outgoing channel, skipping it");
                }
                receive_next_command = msg_fut_continue; // Continue receiving the WebSocket message.
                send_outgoing = r.recv(); // Wait for next outgoing message.
            }
        }
    }

    for jh in remaining_join_handles {
        // TODO: find a way to do the cleanup earlier?
        jh.await;
    }

    Ok(())
}

async fn run() {
    env_logger::init();

    let addr = "127.0.0.1:29031";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        task::spawn(accept_connection(peer, stream));
    }
}

fn main() {
    task::block_on(run());
}
