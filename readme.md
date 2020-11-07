### Test/demo instructions:

```
cargo run --example webserver
```

start node debugger in vscode with `F5`
go to debug console or alternativly you can start `typescript/websocket-example.js` in your browser and use its console.

```js
dc.openContext().log();
logEvents();
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
