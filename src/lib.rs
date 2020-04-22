use deltachat_command_derive::{api_function, get_args_struct};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Command {
    command_id: u32,
    invocation_id: u32,
}

#[derive(Serialize, Debug)]
struct ErrorInstance {
    kind: ErrorType,
    message: String,
    invocation_id: u32,
}

#[derive(Serialize, Debug)]
enum ErrorType {
    CommandIdMissing,
    CommandNotFound,
    CommandParseFailure,
}

pub fn run_json(command: &str) -> String {
    return {
        if let Ok(cmd) = serde_json::from_str::<Command>(command) {
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
                0 => command!(info),
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
        } else {
            serde_json::to_string(&ErrorInstance {
                kind: ErrorType::CommandIdMissing,
                message: "You need to specify a commandId".to_owned(),
                invocation_id: 0,
            })
            .unwrap()
        }
    };
}

#[derive(Serialize, Debug)]
struct Info {
    sample_version: u8,
    sample_info: String,
}

api_function!(
    fn info() -> Info {
        Info {
            sample_version: 9,
            sample_info: "Sample Info".to_owned(),
        }
    }
);
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
            run_json("{ \"command_id\": 1, \"message\": \"Hello Echo\"}"),
            "{\"message\":\"Hello Echo\"}"
        );
    }

    #[test]
    fn errors() {
        assert_eq!(
            run_json("{}"),
            "{\"kind\":\"CommandIdMissing\",\"message\":\"You need to specify a commandId\"}"
        );
        assert_eq!(
            run_json("{ \"command_id\": 0}"),
            "{\"kind\":\"CommandNotFound\",\"message\":\"command with the id 2 not found\"}"
        );
        assert_eq!(
            run_json("{ \"command_id\": 1}"),
            "{\"kind\":\"CommandParseFailure\",\"message\":\"command arguments invalid: Some(Error(\\\"missing field `message`\\\", line: 1, column: 18))\"}"
        );
    }
}
