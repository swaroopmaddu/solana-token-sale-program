import { PublicKey } from "@solana/web3.js";

//@ts-expect-error missing types
import * as BufferLayout from "buffer-layout";

export const TokenSaleAccountLayout = BufferLayout.struct([
  BufferLayout.u8("isInitialized"), //1byte
  BufferLayout.blob(32, "sellerPubkey"), //pubkey(32byte)
  BufferLayout.blob(32, "tempTokenAccountPubkey"), //pubkey(32byte)
  BufferLayout.blob(8, "swapSolAmount"), //8byte
  BufferLayout.blob(8, "swapTokenAmount"), //8byte
]);

export interface TokenSaleAccountLayoutInterface {
  [index: string]: number | Uint8Array;
  isInitialized: number;
  sellerPubkey: Uint8Array;
  tempTokenAccountPubkey: Uint8Array;
  swapSolAmount: Uint8Array;
  swapTokenAmount: Uint8Array;
}

export interface ExpectedTokenSaleAccountLayoutInterface {
  [index: string]: number | PublicKey;
  isInitialized: number;
  sellerPubkey: PublicKey;
  tempTokenAccountPubkey: PublicKey;
  swapSolAmount: number;
  swapTokenAmount: number;
}
