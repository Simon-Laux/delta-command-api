const {
  DeltaChat,
  WebsocketTransport,
  JSONTransport
} = require("./dist/index");

let websocket = new WebsocketTransport(
  "ws://localhost:29031",
  new JSONTransport()
);

const dc = new DeltaChat(websocket);

websocket.setup();

global.dc = dc;

setInterval(() => {}, 100000);

async function logEvents() {
  let context = dc.context;
  /** @type {string} */
  let ev;
  while ((ev = await context._get_next_event_as_string())) {
    if (ev.includes("Info")) {
      console.info(ev);
    } else if (ev.includes("Warning")) {
      console.warn(ev);
    } else if (ev.includes("Error")) {
      console.error(ev);
    } else {
      console.debug(ev);
    }
  }
  // TODO: somehow make sure that this gets restarted when the promise fails.
}

global.logEvents = logEvents;

Promise.prototype.log = function () {
  this.then(console.log).catch(console.error);
  return this;
};
