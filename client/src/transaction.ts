/* eslint-disable @typescript-eslint/no-non-null-assertion */
import * as dotenv from "dotenv";
dotenv.config();

import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
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
import { AccountLayout, Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";

type InstructionNumber = 0 | 1;

const transaction = async () => {
  //phase1 (setup Transaction & send Transaction)
  console.log("Setup Transaction");
  const connection = new Connection("http://localhost:8899", "confirmed");
  const tokenSaleProgramId = new PublicKey(process.env.CUSTOM_PROGRAM_ID!);
  const userPubkey = new PublicKey(process.env.USER_PUBLIC_KEY!);
  const userPrivateKey = Uint8Array.from(JSON.parse(process.env.USER_PRIVATE_KEY!));
  const userWallet = new Keypair({
    publicKey: userPubkey.toBytes(),
    secretKey: userPrivateKey,
  });
  const tokenMintAccountPubkey = new PublicKey(process.env.TOKEN_MINT_ACCOUNT_PUBKEY!);
  const userTokenAccountPubkey = new PublicKey(process.env.USE_TOKEN_ACCOUNT_PUBKEY!);

  const instruction: InstructionNumber = 0;
  const amountOfTokenWantToSale = 10;
  const swapSolAmount = 0.1; // 이거 소수점 처리가능한가?
  const swapTokenAmount = 10;

  //유저의 임시 토큰 account 생성 트랜잭션 (token program으로 생성하면 안되나?)
  const tempTokenAccountKeypair = new Keypair();
  const createTempTokenAccountIx = SystemProgram.createAccount({
    fromPubkey: userWallet.publicKey,
    newAccountPubkey: tempTokenAccountKeypair.publicKey,
    lamports: await connection.getMinimumBalanceForRentExemption(AccountLayout.span),
    space: AccountLayout.span,
    programId: TOKEN_PROGRAM_ID,
  });

  //임시 토큰 초기화 트랜잭션 (이거 왜 필요하지? = 토큰 어카운트에 mint account 연결시켜주기)
  const initTempTokenAccountIx = Token.createInitAccountInstruction(
    TOKEN_PROGRAM_ID,
    tokenMintAccountPubkey,
    tempTokenAccountKeypair.publicKey,
    userWallet.publicKey
  );

  //판매할 수량만큼 토큰 송금 (왜 ATA를 사용해서는 보낼 수 없는거지? = 보낼 수 있는듯?)
  const transferTokenToTempTokenAccountIx = Token.createTransferInstruction(
    TOKEN_PROGRAM_ID,
    userTokenAccountPubkey,
    tempTokenAccountKeypair.publicKey,
    userWallet.publicKey,
    [],
    amountOfTokenWantToSale
  );

  const tokenSaleProgramAccountKeypair = new Keypair();
  const createTokenSaleProgramAccountIx = SystemProgram.createAccount({
    fromPubkey: userWallet.publicKey,
    newAccountPubkey: tokenSaleProgramAccountKeypair.publicKey,
    lamports: await connection.getMinimumBalanceForRentExemption(TokenSaleAccountLayout.span),
    space: TokenSaleAccountLayout.span,
    programId: tokenSaleProgramId,
  });

  const initTokenSaleProgramIx = new TransactionInstruction({
    programId: tokenSaleProgramId,
    keys: [
      createAccountInfo(userWallet.publicKey, true, false),
      createAccountInfo(tempTokenAccountKeypair.publicKey, false, true),
      createAccountInfo(tokenSaleProgramAccountKeypair.publicKey, false, true),
      createAccountInfo(SYSVAR_RENT_PUBKEY, false, false),
      createAccountInfo(TOKEN_PROGRAM_ID, false, false),
    ],
    data: Buffer.from(
      Uint8Array.of(
        instruction,
        ...new BN(swapSolAmount).toArray("le", 8),
        ...new BN(swapTokenAmount).toArray("le", 8)
      )
    ),
  });

  console.log(
    Buffer.from(
      Uint8Array.of(
        instruction,
        ...new BN(swapSolAmount).toArray("le", 8),
        ...new BN(swapTokenAmount).toArray("le", 8)
      )
    )
  );

  //make transaction with several instructions(ix)
  console.log("Send transaction...\n");
  const tx = new Transaction().add(
    createTempTokenAccountIx,
    initTempTokenAccountIx,
    transferTokenToTempTokenAccountIx,
    createTokenSaleProgramAccountIx,
    initTokenSaleProgramIx
  );

  await connection.sendTransaction(
    tx,
    [userWallet, tempTokenAccountKeypair, tokenSaleProgramAccountKeypair],
    {
      skipPreflight: false,
      preflightCommitment: "confirmed",
    }
  );
  //phase1 end

  //wait block update
  await new Promise((resolve) => setTimeout(resolve, 1000));

  //phase2 (check Transaction result is valid)
  const tokenSaleProgramAccount = await checkAccountInitialized(
    connection,
    tokenSaleProgramAccountKeypair.publicKey
  );

  const encodedTokenSaleProgramAccountData = tokenSaleProgramAccount.data;
  const decodedTokenSaleProgramAccountData = TokenSaleAccountLayout.decode(
    encodedTokenSaleProgramAccountData
  ) as TokenSaleAccountLayoutInterface;

  const expectedTokenSaleProgramAccountData: ExpectedTokenSaleAccountLayoutInterface = {
    isInitialized: 1,
    walletPubkey: userWallet.publicKey,
    tempTokenAccountPubkey: tempTokenAccountKeypair.publicKey,
    swapSolAmount: swapSolAmount,
    swapTokenAmount: swapTokenAmount,
  };

  console.log("Current AccountData");
  checkAccountDataIsValid(decodedTokenSaleProgramAccountData, expectedTokenSaleProgramAccountData);

  //#phase2 end
  console.log(`✨TX successfully finished✨\n`);
};

transaction();
