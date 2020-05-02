use deltachat_command_derive::{api_function, api_function2, get_args_struct};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use deltachat::context::Context;
use deltachat::Event;

mod chatlistitem;
use chatlistitem::*;

pub struct Account {
    pub ctx: std::sync::Arc<Context>,
    pub event_queu: std::sync::Arc<std::sync::RwLock<Vec<Event>>>,
}

#[derive(Serialize, Debug)]
struct Response<T> {
    result: T,
    invocation_id: u32,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    pub kind: ErrorType,
    pub message: String,
    pub invocation_id: u32,
}

fn result_to_string<T: ?Sized>(result: Result<T, ErrorInstance>, invocation_id: u32) -> String
where
    T: Serialize,
    T: std::marker::Sized,
{
    match result {
        Err(e) => serde_json::to_string(&ErrorResponse {
            kind: e.kind,
            message: e.message,
            invocation_id: invocation_id,
        })
        .unwrap(),
        Ok(r) => serde_json::to_string(&Response {
            result: r,
            invocation_id: invocation_id,
        })
        .unwrap(),
    }
}

impl Account {
    pub fn run_json(&self, command: &str, cmd: Command) -> String {
        macro_rules! command {
            ($cmdFunction: expr) => {
                result_to_string(
                    {
                        let result =
                            serde_json::from_str::<get_args_struct!($cmdFunction)>(command);
                        if let Ok(args) = result {
                            $cmdFunction(args, &self)
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
    fn info() -> Result<HashMap<&'static str, std::string::String>, ErrorInstance> {
        Ok(account.ctx.get_info())
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

#[derive(Deserialize, Debug)]
pub struct Command {
    pub command_id: u32,
    pub invocation_id: u32,
}

pub struct ErrorInstance {
    pub kind: ErrorType,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub enum ErrorType {
    CommandIdMissing,
    CommandNotFound,
    CommandNotImplementedYet,
    CommandParseFailure,
    NoContext,
    /** the command threw an Error */
    Generic,
    DeltaChatError,
}

#[derive(Serialize, Debug)]
pub struct SuccessResponse {
    /** this is always true */
    pub success: bool,
    pub invocation_id: u32,
}

pub fn run_json(command: &str, cmd: Command) -> String {
    macro_rules! command {
        ($cmdFunction: expr) => {{
            let result = serde_json::from_str::<get_args_struct!($cmdFunction)>(command);
            if let Ok(args) = result {
                serde_json::to_string(&$cmdFunction(args, cmd.invocation_id)).unwrap()
            } else {
                serde_json::to_string(&ErrorResponse {
                    kind: ErrorType::CommandParseFailure,
                    message: format!("command arguments invalid: {:?}", result.err()),
                    invocation_id: cmd.invocation_id,
                })
                .unwrap()
            }
        }};
    }
    match cmd.command_id {
        1 => command!(echo),
        2 => command!(add),
        3 => command!(subtract),
        _ => serde_json::to_string(&ErrorResponse {
            kind: ErrorType::CommandNotFound,
            message: format!("command with the id {} not found", cmd.command_id),
            invocation_id: cmd.invocation_id,
        })
        .unwrap(),
    }
}

api_function!(
    fn echo<'t>(message: &'t str) -> &'t str {
        message
    }
);

api_function!(
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
);

api_function!(
    fn subtract(a: u32, b: u32) -> u32 {
        a - b
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn echo() {
        assert_eq!(
            run_json(
                "{ \"command_id\": 1, \"message\": \"Hello Echo\", \"invocation_id\": 476}",
                Command {
                    command_id: 1,
                    invocation_id: 476
                }
            ),
            "{\"result\":\"Hello Echo\",\"invocation_id\":476}"
        );
    }

    #[test]
    fn errors() {
        assert_eq!(
            run_json("{ \"command_id\": 1, \"invocation_id\": 0}", Command { command_id: 1, invocation_id: 0}),
            "{\"kind\":\"CommandParseFailure\",\"message\":\"command arguments invalid: Some(Error(\\\"missing field `message`\\\", line: 1, column: 38))\",\"invocation_id\":0}"
        );
    }
}
