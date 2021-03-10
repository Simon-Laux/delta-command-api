use crate::error::{ErrorInstance, ErrorType};
use deltachat_command_derive::{api_function, get_args_struct};
use serde::{Deserialize, Serialize};

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

pub fn result_to_string<T: ?Sized>(result: Result<T, ErrorInstance>, invocation_id: u32) -> Result<String, serde_json::Error>
where
    T: Serialize,
    T: std::marker::Sized,
{
    Ok(match result {
        Err(e) => serde_json::to_string(&ErrorResponse {
            kind: e.kind,
            message: e.message,
            invocation_id,
        })?,
        Ok(r) => serde_json::to_string(&Response {
            result: r,
            invocation_id,
        })?,
    })
}

#[derive(Deserialize, Debug)]
pub struct Command {
    pub command_id: u32,
    pub invocation_id: u32,
}

#[derive(Serialize, Debug)]
pub struct SuccessResponse {
    /** this is always true */
    pub success: bool,
    pub invocation_id: u32,
}

pub fn run_json(command: &str, cmd: Command) -> Result<String, serde_json::Error> {
    macro_rules! command {
        ($cmdFunction: expr) => {{
            let result = serde_json::from_str::<get_args_struct!($cmdFunction)>(command);
            if let Ok(args) = result {
                serde_json::to_string(&$cmdFunction(args, cmd.invocation_id))?
            } else {
                serde_json::to_string(&ErrorResponse {
                    kind: ErrorType::CommandParseFailure,
                    message: format!("command arguments invalid: {:?}", result.err()),
                    invocation_id: cmd.invocation_id,
                })?
            }
        }};
    }
    Ok(match cmd.command_id {
        1 => command!(echo),
        2 => command!(add),
        3 => command!(subtract),
        _ => serde_json::to_string(&ErrorResponse {
            kind: ErrorType::CommandNotFound,
            message: format!("command with the id {} not found", cmd.command_id),
            invocation_id: cmd.invocation_id,
        })?,
    })
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
