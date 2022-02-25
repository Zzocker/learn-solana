import Utils from './utils'
import { struct } from '@solana/buffer-layout'
import { u64 } from '@solana/buffer-layout-utils'
import { PublicKey, TransactionInstruction, sendAndConfirmTransaction, Transaction, Keypair, SystemProgram } from '@solana/web3.js'

const PROGRAM_ID = new PublicKey("FKrr1bp3dZniPuys4oFWnT7yaSCxMRJZD5KH1nWmkQJr");

interface InstructionData {
    amount: bigint
}

const instructionData = struct<InstructionData>([u64('amount')]);

async function main() {
    const connection = await Utils.getConnection(Utils.rup_url);
    const payer = Utils.getPayer();

    const data = Buffer.alloc(instructionData.span);
    instructionData.encode(
        {
            amount: BigInt(100)
        },
        data);

    const instruction = new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { pubkey: new PublicKey("DE33RKYWEpwLpey8RyA9K1NfXJYTnbDZ1Zs87KEfUbxr"), isWritable: true, isSigner: false },
            { pubkey: new PublicKey("24yDoNGW6196W8v8JERiQYtPjDzbQYpEKpJHiCAcPQrS"), isWritable: true, isSigner: false }
        ],
        data: data
    })

    await sendAndConfirmTransaction(connection, new Transaction().add(instruction), [payer]);
}

main()
    .then(() => process.exit(0))
    .catch((err) => {
        console.error(err)
        process.exit(1)
    })