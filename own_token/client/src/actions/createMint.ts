import { PublicKey, Connection, Keypair, ConfirmOptions, Transaction, sendAndConfirmTransaction, SendTransactionError, SystemProgram, SystemInstruction } from '@solana/web3.js'
import { TOKEN_PROGRAM_ID } from '../constants'
import { createInitializeMintInstruction } from '../index'
import { getMinimumBalanceForRentExemptMint, MintLayout } from '../state/mint'

export async function createMint(
    connection: Connection,
    payer: Keypair,
    mintAuthority: PublicKey,
    decimals: number,
    freezeAuthority: PublicKey | null,
    confirmOptions?: ConfirmOptions,
    keypair = Keypair.generate(),
    programID = TOKEN_PROGRAM_ID
): Promise<PublicKey> {
    const lamports = await getMinimumBalanceForRentExemptMint(connection);
    const tx = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: keypair.publicKey,
            lamports: lamports,
            space: MintLayout.span,
            programId: programID
        }),
        createInitializeMintInstruction(keypair.publicKey,programID,mintAuthority,decimals,freezeAuthority)
    )

    await sendAndConfirmTransaction(connection, tx, [payer,keypair], confirmOptions);
    return keypair.publicKey
}