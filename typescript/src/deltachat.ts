import {TransportMethod} from "./transportMethod";

export class DeltaChat {

    constructor (
        public transport:TransportMethod
    ){}

    
    async getInfo () {
        return this.transport.send(0, {})
    }

    async echo (message:string) {
        return this.transport.send(0, {message})
    }
}