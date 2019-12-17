use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Command {
    command_id: u32,
}

#[derive(Serialize, Debug)]
struct ErrorInstance<'t> {
    kind: ErrorType,
    message: String,
}

#[derive(Serialize, Debug)]
enum ErrorType {
    CommandIdMissing,
    CommandNotFound,
    CommandParseFailure
}

pub fn run_json(command: &str) -> String {
    return serde_json::to_string({
        if let Ok(cmd) = serde_json::from_str::<Command>(command){
            match cmd.command_id {
                1 => {
                    let result = serde_json::from_str::<EchoCommand>(command);
                    if let Ok(echo_args) = result {
                        &echo(echo_args)
                    } else {
                        &ErrorInstance {
                            kind: ErrorType::CommandParseFailure,
                            message: format!("command with the id {} not found", result.err().message ),
                        }
                    }
                }
                _ => &ErrorInstance {
                    kind: ErrorType::CommandNotFound,
                    message: format!("command with the id {} not found", cmd.command_id ),
                }
            }

        } else {
            &ErrorInstance {
                kind: ErrorType::CommandIdMissing,
                message: "You need to specify a commandId".to_owned(),
            }
        }
    })
    .unwrap();
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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
