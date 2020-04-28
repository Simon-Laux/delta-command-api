use delta_command_api::*;
use std::env::current_dir;
use std::env::vars;
use std::sync::{atomic, Arc, RwLock};
use std::thread;

use deltachat::chat::*;
use deltachat::config;
use deltachat::constants::{Chattype, Viewtype, DC_CONTACT_ID_SELF};
use deltachat::context::*;
use deltachat::error::Error;
use deltachat::job::{
    perform_inbox_fetch, perform_inbox_idle, perform_inbox_jobs, perform_smtp_idle,
    perform_smtp_jobs,
};
use deltachat::message::*;
use deltachat::Event;

// this eample will use websocket+json

use std::net::TcpListener;
use std::thread::spawn;

use tungstenite::accept_hdr;
use tungstenite::handshake::server::Request;

use tungstenite::Message::Text;

fn main() {
    let dbdir = current_dir().unwrap().join("deltachat-db");
    std::fs::create_dir_all(dbdir.clone()).unwrap();
    let dbfile = dbdir.join("db.sqlite");
    println!("creating database {:?}", dbfile);
    let event_queu: RwLock<Vec<Event>> = RwLock::new(Vec::new());
    let evq = Arc::new(event_queu);
    let evq0 = evq.clone();
    let ctx = Context::new(
        Box::new(move |_ctx: &Context, event: Event| {
            // println!("[EV]{:?}", event)

            evq0.write().unwrap().push(event)
        }),
        "FakeOs".into(),
        dbfile,
    )
    .expect("Failed to create context");
    let running = Arc::new(RwLock::new(true));
    // let info = ctx.get_info();
    // println!("info: {:#?}", info);
    let ctx = Arc::new(ctx);
    let ctx0 = ctx.clone();
    let evq1 = evq.clone();
    let account = Account {
        ctx: ctx0,
        event_queu: evq1,
    };
    let ctx1 = ctx.clone();
    let r1 = running.clone();
    let _t1 = thread::spawn(move || {
        while *r1.read().unwrap() {
            perform_inbox_jobs(&ctx1);
            if *r1.read().unwrap() {
                perform_inbox_fetch(&ctx1);

                if *r1.read().unwrap() {
                    perform_inbox_idle(&ctx1);
                }
            }
        }
    });

    let ctx1 = ctx.clone();
    let r1 = running.clone();
    let _t2 = thread::spawn(move || {
        while *r1.read().unwrap() {
            perform_smtp_jobs(&ctx1);
            if *r1.read().unwrap() {
                perform_smtp_idle(&ctx1);
            }
        }
    });

    // println!("configuring");

    // if let Some(addr) = vars().find(|key| key.0 == "addr") {
    //     ctx.set_config(config::Config::Addr, Some(&addr.1)).unwrap();
    // } else {
    //     panic!("no addr ENV var specified");
    // }

    // if let Some(pw) = vars().find(|key| key.0 == "mailpw") {
    //     ctx.set_config(config::Config::MailPw, Some(&pw.1)).unwrap();
    // } else {
    //     panic!("no mailpw ENV var specified");
    // }

    // ctx.configure();

    let acc = Arc::new(account);

    let server = TcpListener::bind("127.0.0.1:29031").unwrap();
    println!("server running on {:?}", server.local_addr());
    for stream in server.incoming() {
        let acc0 = acc.clone();
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
                    let answer = Text(acc0.run_json(msg.to_text().unwrap()));
                    websocket.write_message(answer).unwrap();
                }
            }
        });
    }

    println!("stopping threads");
    *running.write().unwrap() = false;
    deltachat::job::interrupt_inbox_idle(&ctx);
    deltachat::job::interrupt_smtp_idle(&ctx);

    println!("joining");
    _t1.join().unwrap();
    _t2.join().unwrap();

    println!("closing");
}
