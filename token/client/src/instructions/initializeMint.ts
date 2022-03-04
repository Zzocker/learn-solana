import { TokenInstruction } from "./types";
import { PublicKey, TransactionInstruction, AccountMeta, SYSVAR_RENT_PUBKEY } from '@solana/web3.js'
import { struct, u8 } from "@solana/buffer-layout";
import { publicKey } from "@solana/buffer-layout-utils";
import { TOKEN_PROGRAM_ID } from "..";

export interface InitializeMintInstructionData {
    instruction: TokenInstruction.InitializeMint;
    decimals: number;
    mintAuthority: PublicKey;
    freezeAuthorityOption: 1 | 0;
    freezeAuthority: PublicKey
}

export const initializeMintInstructionData = struct<InitializeMintInstructionData>([
    u8("instruction"),
    u8("decimals"),
    publicKey("mintAuthority"),
    u8("freezeAuthorityOption"),
    publicKey("freezeAuthority")
])

export function createInitializeMintInstruction(
    mint: PublicKey,
    decimals: number,
    mintAuthority: PublicKey,
    freezeAuthority: PublicKey | null,
    program_id = TOKEN_PROGRAM_ID
): TransactionInstruction {
    const keys: Array<AccountMeta> = [
        { pubkey: mint, isWritable: true, isSigner: false },
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }
    ]

    const data = Buffer.alloc(initializeMintInstructionData.span);
    initializeMintInstructionData.encode(
        {
            instruction: TokenInstruction.InitializeMint,
            decimals: decimals,
            mintAuthority: mintAuthority,
            freezeAuthorityOption: freezeAuthority ? 1 : 0,
            freezeAuthority: freezeAuthority || new PublicKey(0)
        }, data
    )

    console.log(data)
    return new TransactionInstruction({
        keys: keys,
        programId: program_id,
        data: data
    })
}