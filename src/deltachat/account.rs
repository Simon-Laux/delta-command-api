use crate::commands::{result_to_string, Command};
use crate::error::*;
use deltachat::context::Context;
use deltachat::Event;
use deltachat_command_derive::{api_function2, get_args_struct};
use serde::{Deserialize};
use std::collections::HashMap;

use super::chatlistitem::*;
use super::message::*;

pub struct Account {
    pub ctx: std::sync::Arc<Context>,
    pub event_queu: std::sync::Arc<std::sync::RwLock<Vec<Event>>>,
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
            22 => command!(get_next_event_as_string),
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
    fn get_next_event_as_string() -> Result<Option<String>, ErrorInstance> {
        let mut event_queu = account.event_queu.write().unwrap();
        if event_queu.len() > 0 {
            Ok(Some(format!("{:?}", event_queu.remove(0))))
        } else {
            Ok(None)
        }
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
