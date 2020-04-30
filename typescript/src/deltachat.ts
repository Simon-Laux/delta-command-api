import { TransportMethod } from "./transportMethod";

export class DeltaChat {
  private _context: Context | null;
  constructor(public transport: TransportMethod) {}

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
  constructor(public transport: TransportMethod) {}

  /** Login to an email account */
  async configure(/* TODO */) {
    throw new Error("Not implemented yet");
  }

  /** get information abeout deltachat core and the current context */
  async getInfo(): Promise<{ [key: string]: string }> {
    return this.transport.send(21, {});
  }

  /** get the next event as string */
  async _get_next_event_as_string(): Promise<string> {
    return this.transport.send(22, {});
  }

  /** triggers an error to test error behaviour */
  async _trigger_error(): Promise<boolean> {
    return this.transport.send(500, {});
  }

  async getChatListIds(
    listFlags: number,
    options?: {
      /** search word for searching */
      query?: string;
      queryContactId?: number;
    }
  ): Promise<number[]> {
    return this.transport.send(40, {
      listflags: listFlags,
      query: options.query,
      query_contact_id: options.queryContactId
    });
  }
}
