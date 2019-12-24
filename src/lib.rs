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

macro_rules! command {
    ($cmd: expr, $command: expr, $cmdType: ty, $cmdFunction: expr) => {{
        // are those macro types right?
        let result = serde_json::from_str::<$cmdType>($command);
        if let Ok(args) = result {
            serde_json::to_string(&$cmdFunction(args, $cmd.invocation_id)).unwrap()
        } else {
            serde_json::to_string(&ErrorInstance {
                kind: ErrorType::CommandParseFailure,
                message: format!("command arguments invalid: {:?}", result.err()),
                invocation_id: $cmd.invocation_id,
            })
            .unwrap()
        }
    }};
}

pub fn run_json(command: &str) -> String {
    return {
        if let Ok(cmd) = serde_json::from_str::<Command>(command) {
            match cmd.command_id {
                1 => command!(cmd, command, EchoCommand, echo),
                2 => command!(cmd, command, AddCommand, add),
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

#[derive(Deserialize, Debug)]
struct EchoCommand<'t> {
    message: &'t str,
}

#[derive(Serialize, Debug)]
struct EchoResult<'t> {
    message: &'t str,
    invocation_id: u32,
}

fn echo(args: EchoCommand, invocation_id: u32) -> EchoResult {
    EchoResult {
        message: args.message,
        invocation_id: invocation_id,
    }
}

#[derive(Deserialize, Debug)]
struct AddCommand {
    a: u32,
    b: u32,
}

#[derive(Serialize, Debug)]
struct AddCommandResult {
    result: u32,
    invocation_id: u32,
}

fn add(args: AddCommand, invocation_id: u32) -> AddCommandResult {
    AddCommandResult {
        result: args.a + args.b,
        invocation_id: invocation_id,
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
