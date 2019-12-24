const { DeltaChat, WebsocketTransport, JSONTransport } = require("./dist/index")

let websocket = new WebsocketTransport(
        "ws://localhost:29031",
        new JSONTransport()
    )

const dc = new DeltaChat(
    websocket
)

websocket.setup()

global.dc = dc


setInterval(()=>{}, 100000)