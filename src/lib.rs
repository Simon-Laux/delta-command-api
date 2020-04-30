use deltachat_command_derive::{api_function, api_function2, get_args_struct};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use deltachat::context::Context;
use deltachat::Event;

pub struct Account {
    pub ctx: std::sync::Arc<Context>,
    pub event_queu: std::sync::Arc<std::sync::RwLock<Vec<Event>>>,
}

impl Account {
    pub fn run_json(&self, command: &str, cmd: Command) -> String {
        macro_rules! command {
            ($cmdFunction: expr) => {{
                let result = serde_json::from_str::<get_args_struct!($cmdFunction)>(command);
                if let Ok(args) = result {
                    serde_json::to_string(&$cmdFunction(args, cmd.invocation_id, &self)).unwrap()
                } else {
                    serde_json::to_string(&ErrorInstance {
                        kind: ErrorType::CommandParseFailure,
                        message: format!("command arguments invalid: {:?}", result.err()),
                        invocation_id: cmd.invocation_id,
                    })
                    .unwrap()
                }
            }};
        }
        match cmd.command_id {
            21 => command!(info),
            22 => command!(get_next_event_as_string),
            _ => serde_json::to_string(&ErrorInstance {
                kind: ErrorType::CommandNotFound,
                message: format!("command with the id {} not found", cmd.command_id),
                invocation_id: cmd.invocation_id,
            })
            .unwrap(),
        }
    }
}
api_function2!(
    fn info() -> HashMap<&'static str, std::string::String> {
        account.ctx.get_info()
    }
);
api_function2!(
    fn get_next_event_as_string() -> Option<String> {
        let mut event_queu = account.event_queu.write().unwrap();
        if event_queu.len() > 0 {
            Some(format!("{:?}", event_queu.remove(0)))
        } else {
            None
        }
    }
);

#[derive(Deserialize, Debug)]
pub struct Command {
    pub command_id: u32,
    pub invocation_id: u32,
}

#[derive(Serialize, Debug)]
pub struct ErrorInstance {
    pub kind: ErrorType,
    pub message: String,
    pub invocation_id: u32,
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
                serde_json::to_string(&ErrorInstance {
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
        _ => serde_json::to_string(&ErrorInstance {
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
