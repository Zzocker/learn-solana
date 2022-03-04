import { struct, u32, u8 } from '@solana/buffer-layout'
import { bool, publicKey, u64 } from "@solana/buffer-layout-utils"
import { Commitment, Connection, PublicKey } from '@solana/web3.js'
import { TOKEN_PROGRAM_ID } from ".."
import { TokenAccountNotFoundError, TokenInvalidAccountOwnerError, TokenInvalidAccountSizeError } from '../errors'

// Information about a mint
export interface Mint {
    // address of the mint
    address: PublicKey

    // Optional authority used to mint new token. The mint authority may only be provided during mint creation
    // If ni mint authority is present then the mint has a fixed supply and no further tokens may be minted
    mintAuthority: PublicKey | null

    // Total supply of tokens
    supply: bigint

    // Number of base 10 digits to the right of the decimal place
    decimals: number

    // Is this mint initialized
    isInitialized: boolean

    // Optional authority to freeze TOKEN ACCOUNTS
    freezeAuthority: PublicKey | null
}

// Mint as stored by the program
export interface RawMint {
    mintAuthorityOption: 1 | 0
    mintAuthority: PublicKey
    supply: bigint
    decimals: number
    isInitialized: boolean
    freezeAuthorityOption: 1 | 0
    freezeAuthority: PublicKey
}

// Buffer layout for de/serializing a mint
export const MintLayout = struct<RawMint>(
    [
        u32("mintAuthorityOption"),
        publicKey("mintAuthority"),
        u64("supply"),
        u8("decimals"),
        bool("isInitialized"),
        u32("freezeAuthorityOption"),
        publicKey("freezeAuthority")
    ]
)

// Byte length of a mint
export const MINT_SIZE = MintLayout.span;

export async function getMinimumBalanceForRentExemptMint(connection: Connection, commitment?: Commitment): Promise<number> {
    return await connection.getMinimumBalanceForRentExemption(MINT_SIZE, commitment);
}

export async function getMint(
    connection: Connection,
    address: PublicKey,
    commitment?: Commitment,
    progarmID = TOKEN_PROGRAM_ID
): Promise<Mint> {
    const info = await connection.getAccountInfo(address, commitment);
    if (!info) throw new TokenAccountNotFoundError();
    if (!info.owner.equals(progarmID)) throw new TokenInvalidAccountOwnerError();
    if (info.data.length != MINT_SIZE) throw new TokenInvalidAccountSizeError();

    const rawMint = MintLayout.decode(info.data)

    return {
        address: address,
        mintAuthority: rawMint.mintAuthorityOption ? rawMint.mintAuthority : null,
        supply: rawMint.supply,
        decimals: rawMint.decimals,
        isInitialized: rawMint.isInitialized,
        freezeAuthority: rawMint.freezeAuthorityOption ? rawMint.freezeAuthority : null
    }
}