export interface TransportMethod {
    send(commandId:number, parameters:{[key:string]:any}):Promise<{[key:string]:any}>
}

export class WebsocketTransport implements TransportMethod {
    callbacks:{cb:Function}[]

    constructor(address:string, private format: import("./transportFormat").TransportFormat) {
    
    }
    send(commandId: number, parameters: {[key:string]:any}): Promise<{
        [key: string]: any;
    }> {
        throw new Error("Method not implemented.");
    }

    _currentCallCount() {
        return this.callbacks.length
    }
}
