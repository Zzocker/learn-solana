import { TransactionInstruction,PublicKey,sendAndConfirmTransaction, Transaction } from '@solana/web3.js';
import Utils from './utils'


async function main(){
    const connection = await Utils.getConnection(Utils.rup_url)
    const payer = Utils.getPayer();

    const instruction = new TransactionInstruction({
        keys: [],
        data: Buffer.alloc(0),
        programId: new PublicKey('EUHMfPbM8e9Wpe9hnTkAk6w7fd7fAZL6BweF79ZSR1cP')
    })

    await sendAndConfirmTransaction(connection,new Transaction().add(instruction),[payer]);
}

main()
.then(()=> process.exit(0))
.catch((err)=>{
    console.error(err)
    process.exit(1)
})