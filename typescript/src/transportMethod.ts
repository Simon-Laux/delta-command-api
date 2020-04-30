import WebSocket from "isomorphic-ws";

export interface TransportMethod {
  send(commandId: number, parameters: { [key: string]: any }): Promise<any>;
}

export class WebsocketTransport implements TransportMethod {
  callbacks: { res: Function; rej: Function }[] = [];
  socket: WebSocket;
  initialized = false;
  online = false;

  constructor(
    private address: string,
    private format: import("./transportFormat").TransportFormat
  ) {}

  setup(): Promise<void> {
    return new Promise((res, rej) => {
      this.socket = new WebSocket(this.address);

      this.socket.addEventListener("message", event => {
        // handle answer
        const answer = JSON.parse(event.data);
        // console.log("got", answer)
        if (answer.invocation_id == 0) {
          throw new Error("Command id missing error");
        }
        if (!answer.invocation_id) {
          throw new Error("invocation_id missing");
        }
        const callback = this.callbacks[answer.invocation_id - 1];
        if (!callback) {
          throw new Error(
            `No callback found for invocation_id ${answer.invocation_id}`
          );
        }

        if (answer.kind && answer.message) {
          callback.rej(new Error(`${answer.kind}:${answer.message}`));
        } else {
          callback.res(answer.result || null);
        }

        this.callbacks[answer.invocation_id] = null;
      });
      this.socket.addEventListener("error", event => {
        console.error(event);
        // todo handle error
        this.online = false;
        rej();
      });
      this.socket.addEventListener("close", event => {
        console.debug("socket is closed now");
        this.online = false;
      });
      this.socket.addEventListener("open", event => {
        console.debug("socket is open now");
        this.initialized = true;
        this.online = true;
        res();
      });
    });
  }

  send(
    commandId: number,
    parameters: { [key: string]: any }
  ): Promise<any | null> {
    if (!this.initialized) throw new Error("Socket wasn't initilized yet");
    if (!this.online) throw new Error("Not online");

    let callback;

    const promise = new Promise((res, rej) => {
      callback = { res, rej };
    });

    let data = {
      ...parameters,
      command_id: commandId,
      invocation_id: this.callbacks.push(callback)
    };
    // console.log("sending:", data)
    this.socket.send(this.format.encode(data));
    return promise;
  }

  _currentCallCount() {
    return this.callbacks.length;
  }

  _currentUnresolvedCallCount() {
    return this.callbacks.filter(cb => cb !== null).length;
  }
}
