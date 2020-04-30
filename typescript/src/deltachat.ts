import { TransportMethod } from "./transportMethod";

export class DeltaChat {
  private _context: Context | null;
  constructor(public transport: TransportMethod) {}

  get context() {
    return this._context;
  }

  /** sets the currently active account of the connection */
  async login(/*TODO*/) {
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

  async getInfo() {
    return this.transport.send(21, {});
  }

  async _get_next_event_as_string(): Promise<string> {
    return this.transport.send(22, {});
  }
}
