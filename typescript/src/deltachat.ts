import { TransportMethod } from "./transportMethod";
import { ChatList } from "./deltachat/chatList";
import { DeltaEventEmitter } from "./deltachat/events";

export class DeltaChat extends DeltaEventEmitter {
  private _context: Context | null;
  constructor(public transport: TransportMethod) {
    super();
    transport.setEventEmitter(this.emit.bind(this));
  }

  get context() {
    return this._context;
  }

  /** sets the currently active account of the connection */
  async openContext() {
    await this.transport.send(20, {});
    this._context = new Context(this.transport);
    return this._context;
  }

  // Functions that don't need a context

  async echo(message: string): Promise<string> {
    return this.transport.send(1, { message });
  }

  async add(a: number, b: number): Promise<number> {
    return this.transport.send(2, { a, b });
  }

  async subtract(a: number, b: number): Promise<number> {
    return this.transport.send(3, { a, b });
  }
}

export class Context {
  readonly chatList = new ChatList(this.transport);

  constructor(public transport: TransportMethod) {}

  /** Login to an email account */
  async configure(/* TODO */) {
    throw new Error("Not implemented yet");
  }

  /** get information abeout deltachat core and the current context */
  async getInfo(): Promise<{ [key: string]: string }> {
    return this.transport.send(21, {});
  }

  /** triggers an error to test error behaviour */
  async _trigger_error(): Promise<boolean> {
    return this.transport.send(500, {});
  }

  async getMessageIds(chatId: number): Promise<number[]> {
    return this.transport.send(45, { chat_id: chatId });
  }
}
