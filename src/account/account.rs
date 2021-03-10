use crate::commands::{result_to_string, Command};
use crate::error::*;
use deltachat::context::Context;
use deltachat_command_derive::{api_function2, get_args_struct};
use serde::Deserialize;
use std::collections::HashMap;
use std::env::current_dir;
use std::sync::Arc;

use super::chatlistitem::*;
use super::message::*;

pub struct Account {
    pub ctx: Arc<Context>,
    pub event_channel: deltachat::EventEmitter,
}

impl Account {
    pub async fn run_json(&self, command: &str, cmd: Command) -> String {
        macro_rules! command {
            ($cmdFunction: expr) => {
                result_to_string(
                    {
                        let result =
                            serde_json::from_str::<get_args_struct!($cmdFunction)>(command);
                        if let Ok(args) = result {
                            $cmdFunction(args, &self).await
                        } else {
                            Err(ErrorInstance {
                                kind: ErrorType::CommandParseFailure,
                                message: format!("command arguments invalid: {:?}", result.err()),
                            })
                        }
                    },
                    cmd.invocation_id,
                )
            };
        };

        match cmd.command_id {
            21 => command!(info),
            40 => command!(get_chat_list_ids),
            41 => command!(get_chat_list_items_by_ids),
            45 => command!(get_chat_message_ids),
            46 => command!(get_full_chat_by_id),
            500 => command!(trigger_error),
            _ => result_to_string::<()>(
                Err(ErrorInstance {
                    kind: ErrorType::CommandNotFound,
                    message: format!("command with the id {} not found", cmd.command_id),
                }),
                cmd.invocation_id,
            ),
        }
    }

    pub async fn open() -> Result<Account, anyhow::Error> {
        let dbdir = current_dir().unwrap().join("deltachat-db");
        std::fs::create_dir_all(dbdir.clone())?;
        let dbfile = dbdir.join("db.sqlite");
        println!("creating database {:?}", dbfile);
        let ctx = Context::new("FakeOs".into(), dbfile.into(), 0)
            .await
            .expect("Failed to create context");
        let info = ctx.get_info().await;
        println!("info: {:#?}", info);
        let ctx = Arc::new(ctx);
        println!("------ RUN ------");
        ctx.start_io().await;

        let event_channel = ctx.get_event_emitter();

        Ok(Account {
            ctx,
            event_channel,
        })
    }

    pub async fn close_context(&self) {
        println!("stopping");
        self.ctx.stop_io().await;
        println!("closing");
        while let Some(event) = self.event_channel.recv().await {
            println!("ignoring event {:?}", event);
        }
    }
}

api_function2!(
    async fn info() -> Result<HashMap<&'static str, std::string::String>, ErrorInstance> {
        let info_btreemap = account.ctx.get_info().await;
        let mut hash_map = HashMap::new();

        for (key, value) in info_btreemap {
            hash_map.insert(key, value);
        }

        Ok(hash_map)
    }
);

api_function2!(
    fn trigger_error() -> Result<bool, ErrorInstance> {
        Err(ErrorInstance {
            kind: ErrorType::Generic,
            message: "This function is meant to test the error behaviour".to_owned(),
        })
    }
);
