use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Command {
    command_id: u32,
}

#[derive(Serialize, Debug)]
struct ErrorInstance {
    kind: ErrorType,
    message: String,
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
            match cmd.command_id {
                1 => {
                    let result = serde_json::from_str::<EchoCommand>(command);
                    if let Ok(echo_args) = result {
                        serde_json::to_string(&echo(echo_args)).unwrap()
                    } else {
                        serde_json::to_string(&ErrorInstance {
                            kind: ErrorType::CommandParseFailure,
                            message: format!("command arguments invalid: {:?}", result.err()),
                        })
                        .unwrap()
                    }
                }
                _ => serde_json::to_string(&ErrorInstance {
                    kind: ErrorType::CommandNotFound,
                    message: format!("command with the id {} not found", cmd.command_id),
                })
                .unwrap(),
            }
        } else {
            serde_json::to_string(&ErrorInstance {
                kind: ErrorType::CommandIdMissing,
                message: "You need to specify a commandId".to_owned(),
            })
            .unwrap()
        }
    };
}

#[derive(Deserialize, Debug)]
struct EchoCommand<'t> {
    message: &'t str,
}

#[derive(Serialize, Debug)]
struct EchoResult<'t> {
    message: &'t str,
}

fn echo(args: EchoCommand) -> EchoResult {
    EchoResult {
        message: args.message,
    }
}

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
