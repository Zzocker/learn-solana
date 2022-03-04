import { Connection, clusterApiUrl, Keypair } from '@solana/web3.js'
import { join } from 'path'
import { homedir } from 'os'
import { readFileSync } from 'fs';

import { createMint } from '../src'

async function main() {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    const payer = Keypair.fromSecretKey(Buffer.from(JSON.parse(readFileSync(join(homedir(), ".config", "solana", "id.json"), "utf-8"))))

    // Create new token mint
    const mint = await createMint(
        connection,
        payer,
        payer.publicKey,
        null,
        9
    );

    console.log(`mint address: ${mint.toBase58()}`)
}

main()
    .then(() => process.exit(0))
    .catch((err) => {
        console.error(err);
        process.exit(1);
    })