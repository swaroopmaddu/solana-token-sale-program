/* eslint-disable @typescript-eslint/no-non-null-assertion */
import * as dotenv from "dotenv";
dotenv.config();

import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const setup = async () => {
  const connection = new Connection("http://localhost:8899", "confirmed");
  const userPubkey = new PublicKey(process.env.USER_PUBLIC_KEY!);
  const userPrivateKey = Uint8Array.from(JSON.parse(process.env.USER_PRIVATE_KEY!));
  const userWallet = new Keypair({
    publicKey: userPubkey.toBytes(),
    secretKey: userPrivateKey,
  });

  console.log("Create Token Mint Account...\n");
  const tokenMintAccount = await Token.createMint(
    connection,
    userWallet,
    userWallet.publicKey,
    null,
    0,
    TOKEN_PROGRAM_ID
  );

  console.log("Create Your Token Account...\n");
  const userTokenAccount = await tokenMintAccount.getOrCreateAssociatedAccountInfo(userPubkey);

  console.log("Mint Token to user...\n");
  await tokenMintAccount.mintTo(userTokenAccount.address, userPubkey, [], 5000);

  const userTokenBalance = await connection.getTokenAccountBalance(
    userTokenAccount.address,
    "confirmed"
  );

  console.table([
    {
      tokenMintAccount: tokenMintAccount.publicKey.toString(),
      userTokenAccount: userTokenAccount.address.toString(),
      userTokenBalance: userTokenBalance.value.amount,
    },
  ]);
  console.log(`✨TX successfully finished✨\n`);
};

setup();
