import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import {
  TokenSaleAccountLayoutInterface,
  ExpectedTokenSaleAccountLayoutInterface,
} from "./account";
import BN = require("bn.js");

export const getKeypair = (publicKey: string, privateKey: Uint8Array) =>
  new Keypair({
    publicKey: new PublicKey(publicKey).toBytes(),
    secretKey: privateKey,
  });

export const getTokenBalance = async (pubkey: PublicKey, connection: Connection) => {
  return parseInt((await connection.getTokenAccountBalance(pubkey)).value.amount);
};

export const createAccountInfo = (pubkey: PublicKey, isSigner: boolean, isWritable: boolean) => {
  return {
    pubkey: pubkey,
    isSigner: isSigner,
    isWritable: isWritable,
  };
};

export const checkAccountInitialized = async (
  connection: Connection,
  customAccountPubkey: PublicKey
) => {
  const customAccount = await connection.getAccountInfo(customAccountPubkey);

  if (customAccount === null || customAccount.data.length === 0) {
    console.log("Account of custom program has not been initialized properly");
    process.exit(1);
  }

  return customAccount;
};

export const checkAccountDataIsValid = (
  customAccountData: TokenSaleAccountLayoutInterface,
  expectedCustomAccountState: ExpectedTokenSaleAccountLayoutInterface
) => {
  const keysOfAccountData = Object.keys(customAccountData);
  const data: { [char: string]: string } = {};

  keysOfAccountData.forEach((key) => {
    const value = customAccountData[key];
    const expectedValue = expectedCustomAccountState[key];

    //PublicKey
    if (value instanceof Uint8Array && expectedValue instanceof PublicKey) {
      if (!new PublicKey(value).equals(expectedValue)) {
        console.log(`${key} is not matched expected one`);
        process.exit(1);
      }
    } else if (value instanceof Uint8Array && typeof expectedValue === "number") {
      //value is undefined
      if (!value) {
        console.log(`${key} flag has not been set`);
        process.exit(1);
      }

      //value is not matched expected one.
      const isBufferSame = Buffer.compare(
        value,
        Buffer.from(new BN(expectedValue).toArray("le", value.length))
      );

      if (isBufferSame !== 0) {
        console.log(`[${key}] : expected value is ${expectedValue}, but current value is ${value}`);
        process.exit(1);
      }
    }

    data[key] = expectedValue.toString();
  });
  console.table([data]);
};
