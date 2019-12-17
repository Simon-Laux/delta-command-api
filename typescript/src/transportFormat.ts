export interface TransportFormat {
    encode(data: {[key:string]:any}):any
    decode(data: any):{[key:string]:any}
    /**
     * Wether the format used is binary or a string based
     */
    isBinaryFormat:boolean
}

export class JSONTransport implements TransportFormat {
    encode(data: {
        [key: string]: any;
    }) {
        return JSON.stringify(data);
    }
    decode(data: any): {
        [key: string]: any;
    } {
        return JSON.parse(data);
    }
    isBinaryFormat: false;
}
