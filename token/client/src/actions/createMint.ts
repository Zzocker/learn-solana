import { PublicKey, Connection, Keypair, ConfirmOptions, Transaction, SystemProgram, sendAndConfirmTransaction } from "@solana/web3.js"
import { createInitializeMintInstruction, getMinimumBalanceForRentExemptMint, MINT_SIZE } from ".."
import { TOKEN_PROGRAM_ID } from '../constants'

/**
 * Create and initialize a new mint
 * 
 * @param connection        Connection to use
 * @param payer             Payer of the transaction and initialization fees
 * @param mintAuthority     Account or multisig that will control minting
 * @param freezeAuthority   Optional account or multisig that can freeze token accounts
 * @param decimals          Location of the decimal place
 * @param keypair           Optional keypair, defaulting to a new random one
 * @param confirmOptions    Option for confirming the transaction
 * @param programID         Token program account
 */
export async function createMint(
    connection: Connection,
    payer: Keypair,
    mintAuthority: PublicKey,
    freezeAuthority: PublicKey | null,
    decimals: number,
    keypair = Keypair.generate(),
    confirmOptions?: ConfirmOptions,
    programID = TOKEN_PROGRAM_ID
): Promise<PublicKey> {
    const lamports = await getMinimumBalanceForRentExemptMint(connection);

    const transaction = new Transaction().add(
        SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: keypair.publicKey,
            lamports: lamports,
            space: MINT_SIZE,
            programId: programID
        }),
        createInitializeMintInstruction(keypair.publicKey, decimals, mintAuthority, freezeAuthority, programID)
    )

    await sendAndConfirmTransaction(connection, transaction, [payer, keypair], confirmOptions)
    return keypair.publicKey;
}