const {
  DeltaChat,
  WebsocketTransport,
  JSONTransport,
  Event_TypeID
} = require("./dist/index");

let websocket = new WebsocketTransport(
  "ws://localhost:29031",
  new JSONTransport()
);

const dc = new DeltaChat(websocket);

websocket.setup();

global.ws = websocket;

global.dc = dc;

setInterval(() => {}, 100000);

async function logInfoEvents() {
  dc.on(Event_TypeID.INFO, console.info);
  dc.on(Event_TypeID.WARNING, console.warn);
  dc.on(Event_TypeID.ERROR, console.error);
  dc.on(Event_TypeID.ERROR_NETWORK, console.error);
  dc.on(Event_TypeID.ERROR_SELF_NOT_IN_GROUP, console.error);
}

global.logInfoEvents = logInfoEvents;

Promise.prototype.log = function () {
  this.then(console.log).catch(console.error);
  return this;
};

global.bench = async iterations => {
  const unique = Number(Math.floor(Math.random()* 1000000)).toString(36);
  const label = "bench"+ unique
  const t1 = Date.now();
  console.time(label)
  for (let i = 0; i < iterations; i++) {
    await dc.add(1,4)
  }
  console.timeEnd(label)
  const t2 = Date.now();
  console.log((t2-t1)/iterations)
};


global.pbench = async iterations => {
  const unique = Number(Math.floor(Math.random()* 1000000)).toString(36);
  const label = "start"+ unique
  const label2 = "result"+ unique
  const t1 = Date.now();
  const promises = [];
  console.time(label)
  for (let i = 0; i < iterations; i++) {
    promises.push(dc.add(1,4))
  }
  console.timeEnd(label)
  console.time(label2)
  await Promise.all(promises)
  console.timeEnd(label2)
  const t2 = Date.now();
  console.log((t2-t1)/iterations)
};
