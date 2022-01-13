> **Deprecated**, use the new version of this instead: https://github.com/deltachat/dc_cmd_api

### Test/demo instructions:

run the example server part
```
cargo run --example webserver
```

setup/reset typescript dependencies
(this also already builds the client)
```
./bin/reset_ts.sh
```

build typescript client:
```
cd typescript
npm run build
```

start node debugger in vscode with `F5`
go to debug console or alternativly you can start `typescript/browser-example.html` in your browser and use it's console.

```js
dc.openContext().log();
logInfoEvents();
dc.context.getInfo().log();
dc.context.chatList.getChatListIds(0).log();
(async () =>
  await dc.context.chatList.getChatListItemsByIds(
    await dc.context.chatList.getChatListIds(0)
  ))().log();

(async () =>
  await dc.context.chatList.getFullChatById(
    (await dc.context.chatList.getChatListIds(0))[0]
  ))().log();

(async () =>
  await Promise.all(
    (await dc.context.chatList.getChatListIds(0)).map(id => {
        return dc.context.chatList.getFullChatById(id).log()
    })
  ))().log();
```

the `.log()` helper function logs the result of the promise:

```js
Promise.prototype.log = function() {
  this.then(console.log).catch(console.error);
  return this;
};
```
