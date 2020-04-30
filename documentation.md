## How does it work?

### Protocol

This example will use js instead of json.

Request:

```js
{
    command_id: number,
    invocation_id: number,
    ...args:any[]
}
```

Response:

```js
// success
{
    result: any|null,
    invocation_id: number,
}
// error
{
    kind: ErrorType,
    message: string,
    invocation_id: number,
}
```

`invocation_id` is used as indicator to find out which promise the answer should resolve.

### In rust

1. Try Parse `command_id` and `invocation_id` from the command json
2. Match the command id with the right command
3. then parse the command json as the command-input-struct of that command.
4. give that input struct to the command and stringify the outcoming command response struct.

#### What does the api_function! macro generate?

input:

```rust
api_function!(
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
);
```

output:

```rust
#[derive(Deserialize, Debug)]
struct cmd_add_args {
    a: u32,
    b: u32,
}
#[derive(Serialize, Debug)]
struct cmd_add_res{
    result: u32,
    invocation_id: u32,
}

fn add(args: cmd_info_args, invocation_id: u32) -> cmd_add_res {
    cmd_add_res {
        result: {
            let a = args.a;
            let b = args.a;
            a + b
        },
        invocation_id: invocation_id,
    }
}
```

#### What does the api_function2! macro generate?

Similar to `api_function!` but provides access to a deltachat-context and works with results to express errors.

input:

```rust
api_function2!(
    fn info(_sample_input:bool) -> Result<HashMap<&'static str, std::string::String>, ErrorInstance> {
        Ok(account.ctx.get_info())
    }
);
```

output:

```rust
#[derive(Deserialize, Debug)]
struct cmd_info_args {
    _sample_input:bool
}

fn info(args: cmd_info_args, account: &Account) -> Result<HashMap<&'static str, std::string::String>, ErrorInstance> {
    let _sample_input = args._sample_input;
    account.ctx.get_info()
}
```

#### What does the get_args_struct! macro do?

Thats an easy one, it just creates/gives us an ident that we need for the command macro.

```rust
get_args_struct!(info)
```

becomes:

```rust
cmd_info_args
```
