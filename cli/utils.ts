import {Connection,Keypair} from '@solana/web3.js'
import {join} from 'path'
import {homedir} from 'os'
import { readFileSync } from 'fs';

export default class Utils{
    static rup_url = 'https://api.devnet.solana.com';
    static socket_url = 'wss://api.devnet.solana.com/';
    static keypath = join(homedir(),'.config','solana','id.json')


    static async getConnection(url:string) : Promise<Connection>{
        const connection = new Connection(url,'confirmed');
        const version = await connection.getVersion()
        console.log(`Solana Cluster Version: ${version['solana-core']}`)
        return connection;
    }

    static getPayer(): Keypair{
        const buffer = Buffer.from(JSON.parse(readFileSync(Utils.keypath,"utf-8")))
        return Keypair.fromSecretKey(buffer);
    }
}