import { join } from 'path'
import { homedir } from 'os'
import { readFileSync } from 'fs'
import { Keypair, Connection, clusterApiUrl } from '@solana/web3.js'
import { createMint } from '../src/index'

async function main() {
    const payer = Keypair.fromSecretKey(Buffer.from(JSON.parse(readFileSync(join(homedir(), ".config", "solana", "id.json"), "utf-8"))))
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed")

    const version = await connection.getVersion()
    console.log(`connection version : ${version['solana-core']}`)
    // create mint
    const mint = await createMint(
        connection,
        payer,
        payer.publicKey,
        9,
        null,
    );
    console.log(`mint address : ${mint.toBase58()}`)
}

main()
    .then(() => process.exit(0))
    .catch((err) => {
        console.error(err)
        process.exit(1)
    })