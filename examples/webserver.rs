// this example will use websocket+json
use async_channel::unbounded;
use async_lock::RwLock;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_tungstenite::{
    accept_async,
    tungstenite::{Error, Message},
};
use delta_command_api::account::account::Account;
use delta_command_api::commands::run_json;
use delta_command_api::commands::Command;
use delta_command_api::commands::ErrorResponse;
use delta_command_api::commands::SuccessResponse;
use delta_command_api::error::*;
use futures::future::{select, Either};
use futures::prelude::*;
use log::*;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::anyhow;

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

macro_rules! sendError {
    ($kind: expr, $message: expr, $invocation_id:expr) => {
        Some(serde_json::to_string(&ErrorResponse {
            kind: $kind,
            message: $message.to_owned(),
            invocation_id: $invocation_id,
        }))
    };
}

async fn handle_command(
    connection: &Arc<RwLock<Connection>>,
    text: &str,
    sender: async_channel::Sender<async_tungstenite::tungstenite::Message>,
) -> Option<Result<String, serde_json::Error>> {
    if let Ok(cmd) = serde_json::from_str::<Command>(&text) {
        debug!("cmd id {:?}", cmd);
        match cmd.command_id {
            // if under 20 (no account/context)
            0..=19 => Some(run_json(&text, cmd)),
            // if 20 -> the open context function
            20 => {
                let mut conn = connection.write().await;
                match conn.account {
                    Some(_) => {
                        // make sure active account is NOT set - if set we would theoretically need to close it first.
                        sendError!(
                            ErrorType::Generic,
                            "This connection has already a context opened",
                            cmd.invocation_id
                        )
                    }
                    None => {
                        // handle command and load account (set active account)
                        match conn.open_account(sender).await {
                            Ok(_) => Some(serde_json::to_string(&SuccessResponse {
                                success: true,
                                invocation_id: cmd.invocation_id,
                            })),
                            Err(error) => {
                                error!("Error opening account: {}", error);
                                sendError!(
                                    ErrorType::Generic,
                                    format!("Error opening account: {}", error),
                                    cmd.invocation_id
                                )
                            }
                        }
                    }
                }
            }
            // if over 20 (with account/context)
            _ => {
                // make sure active account is set
                if let Some(ac) = &connection.read().await.account {
                    Some(ac.run_json(&text, cmd).await)
                } else {
                    sendError!(
                        ErrorType::NoContext,
                        "This connection doesn't have a context set: you need to login first",
                        cmd.invocation_id
                    )
                }
            }
        }
    } else {
        sendError!(
            ErrorType::CommandIdMissing,
            "You need to specify a commandId and an invocation id",
            0
        )
    }
}

async fn handle_connection(
    peer: SocketAddr,
    stream: TcpStream,
) -> async_tungstenite::tungstenite::Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // let's say the intervalls are events
    let mut interval = async_std::stream::interval(Duration::from_millis(1000));

    // Echo incoming WebSocket messages and send a message periodically every second.
    let (s, r) = unbounded::<Message>();

    let mut receive_next_command = ws_receiver.next();
    let mut send_outgoing = r.recv();

    let mut connection = Connection::new();
    let conn_arc: Arc<RwLock<Connection>> = Arc::new(RwLock::new(connection));
    loop {
        match select(receive_next_command, send_outgoing).await {
            Either::Left((msg, send_outgoing_continue)) => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        let conn = conn_arc.clone();
                        let stc = s.clone();
                        if msg.is_text() || msg.is_binary() {
                            task::spawn(async move {
                                match msg {
                                    Message::Text(text) => {
                                        println!(":{:?}:", text);
                                        let answer_option =
                                            handle_command(&conn, &text, stc.clone()).await;
                                        info!("answer {:?}", answer_option);
                                        if let Some(answer_result) = answer_option {
                                            match answer_result {
                                                Ok(answer) => match stc
                                                    .send(Message::Text(answer))
                                                    .await
                                                {
                                                    Ok(_) => {}
                                                    Err(err) => error!("error sending {:?}", err),
                                                },
                                                Err(err) => error!("error encoding json {:?}", err),
                                            }
                                        }
                                    }
                                    _ => {
                                        warn!("Recieved unsported message kind: {:?}", msg)
                                    }
                                };
                            });
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

    conn_arc.write().await.cleanup().await;

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

struct Connection {
    pub account: Option<Account>,
    event_sender_task: Option<async_std::task::JoinHandle<()>>,
}

impl Connection {
    fn new() -> Connection {
        Connection {
            account: None,
            event_sender_task: None,
        }
    }

    async fn open_account(
        &mut self,
        sender: async_channel::Sender<async_tungstenite::tungstenite::Message>,
    ) -> anyhow::Result<()> {
        info!("open account");
        if self.account.is_none() {
            let account = Account::open().await?;
            let ctx = account.ctx.clone();

            self.account = Some(account);
            self.event_sender_task = Some(task::spawn(async move {
                while let Some(event) = ctx.get_event_emitter().recv().await {
                    info!("send event {:?}", event);
                    match sender
                        .send(Message::Text(format!(
                            "{{\"event\":true, \"ev\":\"{:?}\"}}",
                            event
                        ))) //TODO real event to string conversion
                        .await
                    {
                        Ok(_) => {}
                        Err(err) => error!("Error sending event {:?}", err),
                    };
                }
            }));
        }
        Ok(())
    }

    async fn cleanup(&self) {
        if let Some(acc) = &self.account {
            acc.close_context().await
        }
    }
}
