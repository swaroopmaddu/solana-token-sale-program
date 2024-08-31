/* eslint-disable @typescript-eslint/no-non-null-assertion */
import * as dotenv from "dotenv";
dotenv.config();

import {
  clusterApiUrl,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import BN = require("bn.js");
import { checkAccountInitialized, checkAccountDataIsValid, createAccountInfo } from "./utils";

import {
  TokenSaleAccountLayout,
  TokenSaleAccountLayoutInterface,
  ExpectedTokenSaleAccountLayoutInterface,
} from "./account";

type InstructionNumber = 0 | 1 | 2 | 3;

const transaction = async () => {
  console.log("Update Token price");

  //phase1 (setup Transaction & send Transaction)
  console.log("Setup Transaction");
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  const tokenSaleProgramId = new PublicKey(process.env.CUSTOM_PROGRAM_ID!);
  const sellerPubkey = new PublicKey(process.env.SELLER_PUBLIC_KEY!);
  const sellerPrivateKey = Uint8Array.from(JSON.parse(process.env.SELLER_PRIVATE_KEY!));
  const sellerKeypair = new Keypair({
    publicKey: sellerPubkey.toBytes(),
    secretKey: sellerPrivateKey,
  });
  const sellerTokenAccountPubkey = new PublicKey(process.env.SELLER_TOKEN_ACCOUNT_PUBKEY!);
  const tokenSaleProgramAccountPubkey = new PublicKey(process.env.TOKEN_SALE_PROGRAM_ACCOUNT_PUBKEY!);
  const tempTokenAccountPubkey = new PublicKey(process.env.TEMP_TOKEN_ACCOUNT_PUBKEY!);
  console.log("sellerTokenAccountPubkey: ", sellerTokenAccountPubkey.toBase58());
  const instruction: InstructionNumber = 3;

  const newPerTokenPrice = 0.0002 * LAMPORTS_PER_SOL;

  const updateTokenPriceIx = new TransactionInstruction({
    programId: tokenSaleProgramId,
    keys: [
      createAccountInfo(sellerKeypair.publicKey, true, false),
      createAccountInfo(tokenSaleProgramAccountPubkey, false, true),
    ],
    data: Buffer.from(Uint8Array.of(instruction, ...new BN(newPerTokenPrice).toArray("le", 8))),
  });

  //make transaction with several instructions(ix)
  console.log("Send transaction...\n");
  const tx = new Transaction().add(updateTokenPriceIx);

  await connection.sendTransaction(tx, [sellerKeypair], {
    skipPreflight: false,
    preflightCommitment: "confirmed",
  });
  //phase1 end

  //wait block update
  await new Promise((resolve) => setTimeout(resolve, 2000));

  //phase2 (check Transaction result is valid)
  const tokenSaleProgramAccount = await checkAccountInitialized(connection, tokenSaleProgramAccountPubkey);

  const encodedTokenSaleProgramAccountData = tokenSaleProgramAccount.data;
  const decodedTokenSaleProgramAccountData = TokenSaleAccountLayout.decode(
    encodedTokenSaleProgramAccountData
  ) as TokenSaleAccountLayoutInterface;

  const expectedTokenSaleProgramAccountData: ExpectedTokenSaleAccountLayoutInterface = {
    isInitialized: 1,
    sellerPubkey: sellerKeypair.publicKey,
    tempTokenAccountPubkey: tempTokenAccountPubkey,
    pricePerToken: newPerTokenPrice,
  };

  console.log("Current TokenSaleProgramAccountData");
  checkAccountDataIsValid(decodedTokenSaleProgramAccountData, expectedTokenSaleProgramAccountData);

  console.table([
    {
      pricePerToken: newPerTokenPrice / LAMPORTS_PER_SOL + " SOL",
    },
  ]);
  console.log(`✨TX successfully finished✨\n`);
  //#phase2 end

};

transaction();
