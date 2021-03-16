import WebSocket from "isomorphic-ws";
import { backend_json_event } from "./deltachat/events";

export interface TransportMethod {
  setEventEmitter(cb: (data: backend_json_event) => void): void;
  initialized: boolean;
  online: boolean;
  send(commandId: number, parameters: { [key: string]: any }): Promise<any>;
}

export class WebsocketTransport implements TransportMethod {
  callbacks: { [invocation_id: number]: { res: Function; rej: Function } } = {};
  invocation_id_counter = 1;
  socket: WebSocket;
  initialized = false;
  online = false;
  eventEmitter: (data: backend_json_event) => void = _data => {};

  constructor(
    private address: string,
    private format: import("./transportFormat").TransportFormat
  ) {}

  private onMessage(event: { data: any; type: string; target: WebSocket }) {
    // handle answer
    // console.log(event.data);
    // return;
    let answer;
    try {
      answer = JSON.parse(event.data);
    } catch (error) {
      console.log("message recieved is not json:", event.data, error);
      return;
    }
    // console.log("got", answer)
    if (answer.event) {
      this.eventEmitter(answer);
    } else {
      if (answer.invocation_id == 0) {
        throw new Error("Command id missing error");
      }
      if (!answer.invocation_id) {
        throw new Error("invocation_id missing");
      }
      const callback = this.callbacks[answer.invocation_id];
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
    }
  }

  setup(): Promise<void> {
    return new Promise((res, rej) => {
      this.socket = new WebSocket(this.address);
      const self = this; // socket event callback overwrites this to undefined sometimes

      this.socket.addEventListener("message", this.onMessage.bind(self));
      this.socket.addEventListener("error", event => {
        console.error(event);
        // todo handle error
        self.online = false;
        rej("socket error");
      });
      this.socket.addEventListener("close", event => {
        console.debug("socket is closed now");
        self.online = false;
      });
      this.socket.addEventListener("open", event => {
        console.debug("socket is open now");
        self.initialized = true;
        self.online = true;
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

    const invocation_id = this.invocation_id_counter++;
    this.callbacks[invocation_id] = callback;
    let data = {
      ...parameters,
      command_id: commandId,
      invocation_id
    };
    // console.log("sending:", data)
    this.socket.send(this.format.encode(data));
    return promise;
  }

  _currentCallCount() {
    return Object.keys(this.callbacks).length;
  }

  _currentUnresolvedCallCount() {
    return Object.keys(this.callbacks).filter(
      key => this.callbacks[Number(key)] !== null
    ).length;
  }

  setEventEmitter(cb: (data: backend_json_event) => void): void {
    this.eventEmitter = cb;
  }
}
