import { PublicKey, TransactionInstruction } from '@solana/web3.js'
import { TokenInstructions } from './types'
import { struct, u8 } from '@solana/buffer-layout'
import { publicKey } from '@solana/buffer-layout-utils'

export interface InitializeMintInstruction {
    method: TokenInstructions.InitializeMint;
    mintAuthority: PublicKey
    decimals: number
    freezeAuthorityOption: 1 | 0
    freezeAuthority: PublicKey
}

export const initializeMintInstruction = struct<InitializeMintInstruction>([
    u8("method"),
    publicKey("mintAuthority"),
    u8("decimals"),
    u8("freezeAuthorityOption"),
    publicKey("freezeAuthority")
])

export function createInitializeMintInstruction(
    address:PublicKey,
    programID: PublicKey,
    mintAuthority: PublicKey,
    decimals: number,
    freezeAuthority: PublicKey | null
): TransactionInstruction {
    const data = Buffer.alloc(initializeMintInstruction.span);
    initializeMintInstruction.encode({
        method: TokenInstructions.InitializeMint,
        mintAuthority: mintAuthority,
        decimals: decimals,
        freezeAuthorityOption: freezeAuthority ? 1 : 0,
        freezeAuthority: freezeAuthority ? freezeAuthority : new PublicKey(0)
    }, data)
    return new TransactionInstruction({
        keys: [{pubkey: address,isSigner : false, isWritable: true}],
        data: data,
        programId: programID
    })
}