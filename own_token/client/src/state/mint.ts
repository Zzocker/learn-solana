import {Commitment, Connection, PublicKey} from '@solana/web3.js'
import {struct, u8} from '@solana/buffer-layout'
import { publicKey, u64 } from '@solana/buffer-layout-utils'

export interface Mint{
    address: PublicKey;
    isInitialized: boolean;
    supply: bigint;
    decimals: number;
    mint_authority: PublicKey | null;
    freeze_authority: PublicKey | null;
}

interface RawMint{
    isInitialized: 1 | 0;
    supply: bigint;
    decimals: number;
    mint_authority_option: 1 | 0;
    mint_authority : PublicKey;
    freeze_authority_option: 1 | 0;
    freeze_authority: PublicKey;
}

export const MintLayout = struct<RawMint>([
    u8("isInitialized"),
    u64("supply"),
    u8("decimals"),
    u8("mint_authority_option"),
    publicKey("mint_authority"),
    u8("freeze_authority_option"),
    publicKey("freeze_authority")
])

export function getMinimumBalanceForRentExemptMint(connection: Connection,commitment?: Commitment): Promise<number>{
    return connection.getMinimumBalanceForRentExemption(MintLayout.span,commitment);
}