const { DeltaChat, WebsocketTransport, JSONTransport } = require("./dist/index")

const dc = new DeltaChat(
    new WebsocketTransport(
        "localhost",
        new JSONTransport()
    )
)

global.dc = dc


setInterval(()=>{}, 100000)