import bs58 from "bs58";
import {
  SystemProgram,
  StakeProgram,
  VOTE_PROGRAM_ID,
  BPF_LOADER_PROGRAM_ID,
  BPF_LOADER_DEPRECATED_PROGRAM_ID,
  SYSVAR_CLOCK_PUBKEY,
  SYSVAR_RENT_PUBKEY,
  SYSVAR_REWARDS_PUBKEY,
  SYSVAR_STAKE_HISTORY_PUBKEY,
  ParsedTransaction,
  TransactionInstruction,
  Transaction,
  PartiallyDecodedInstruction,
  ParsedInstruction,
  Secp256k1Program,
} from "@safecoin/web3.js";
import { Cluster } from "providers/cluster";
import { SerumMarketRegistry } from "serumMarketRegistry";
import { TokenInfoMap } from "@safecoin/safe-token-registry";

export type ProgramName =
  typeof PROGRAM_NAME_BY_ID[keyof typeof PROGRAM_NAME_BY_ID];

export enum PROGRAM_NAMES {
  // native built-ins
  ADDRESS_MAP = "Address Map Program",
  CONFIG = "Config Program",
  STAKE = "Stake Program",
  SYSTEM = "System Program",
  VOTE = "Vote Program",
  SECP256K1 = "Secp256k1 Program",

  // spl
  ASSOCIATED_TOKEN = "Associated Token Program",
  FEATURE_PROPOSAL = "Feature Proposal Program",
  LENDING = "Lending Program",
  MEMO = "Memo Program",
  MEMO_2 = "Memo Program v2",
  NAME = "Name Service Program",
  STAKE_POOL = "Stake Pool Program",
  SWAP = "Swap Program",
  TOKEN = "Token Program",
  TOKEN_METADATA = "Token Metadata Program",
  TOKEN_VAULT = "Token Vault Program",

  // other
  ACUMEN = "Acumen Program",
  BONFIDA_POOL = "Bonfida Pool Program",
  BREAK_SAFECOIN = "Break Safecoin Program",
  MANGO_GOVERNANCE = "Mango Governance Program",
  MANGO_ICO = "Mango ICO Program",
  MANGO_1 = "Mango Program v1",
  MANGO_2 = "Mango Program v2",
  MANGO_3 = "Mango Program v3",
  MARINADE = "Marinade Staking Program",
  MERCURIAL = "Mercurial Stable Swap Program",
  METAPLEX = "Metaplex Program",
  NFT_AUCTION = "NFT Auction Program",
  NFT_CANDY_MACHINE = "NFT Candy Machine Program",
  ORCA_SWAP_1 = "Orca Swap Program v1",
  ORCA_SWAP_2 = "Orca Swap Program v2",
  ORCA_AQUAFARM = "Orca Aquafarm Program",
  PORT = "Port Finance Program",
  PYTH = "Pyth Oracle Program",
  QUARRY_MERGE_MINE = "Quarry Merge Mine",
  QUARRY_MINE = "Quarry Mine",
  QUARRY_MINT_WRAPPER = "Quarry Mint Wrapper",
  QUARRY_REDEEMER = "Quarry Redeemer",
  QUARRY_REGISTRY = "Quarry Registry",
  RAYDIUM_AMM = "Raydium AMM Program",
  RAYDIUM_IDO = "Raydium IDO Program",
  RAYDIUM_LP_1 = "Raydium Liquidity Pool Program v1",
  RAYDIUM_LP_2 = "Raydium Liquidity Pool Program v2",
  RAYDIUM_STAKING = "Raydium Staking Program",
  SABER_ROUTER = "Saber Router Program",
  SABER_SWAP = "Saber Stable Swap Program",
  SERUM_1 = "Serum Dex Program v1",
  SERUM_2 = "Serum Dex Program v2",
  SERUM_3 = "Serum Dex Program v3",
  SERUM_SWAP = "Serum Swap Program",
  SAFEEND = "Safeend Program",
  SAFEIDO = "Lido for Safecoin Program",
  STEP_SWAP = "Step Finance Swap Program",
  SWITCHBOARD = "Switchboard Oracle Program",
  WORMHOLE = "Wormhole",
}

const ALL_CLUSTERS = [
  Cluster.Custom,
  Cluster.Devnet,
  Cluster.Testnet,
  Cluster.MainnetBeta,
];

const LIVE_CLUSTERS = [Cluster.Devnet, Cluster.Testnet, Cluster.MainnetBeta];
const MAINNET_ONLY = [Cluster.MainnetBeta];

export const PROGRAM_DEPLOYMENTS = {
  // native built-ins
  [PROGRAM_NAMES.ADDRESS_MAP]: ALL_CLUSTERS,
  [PROGRAM_NAMES.CONFIG]: ALL_CLUSTERS,
  [PROGRAM_NAMES.STAKE]: ALL_CLUSTERS,
  [PROGRAM_NAMES.SYSTEM]: ALL_CLUSTERS,
  [PROGRAM_NAMES.VOTE]: ALL_CLUSTERS,
  [PROGRAM_NAMES.SECP256K1]: ALL_CLUSTERS,

  // spl
  [PROGRAM_NAMES.ASSOCIATED_TOKEN]: ALL_CLUSTERS,
  [PROGRAM_NAMES.FEATURE_PROPOSAL]: ALL_CLUSTERS,
  [PROGRAM_NAMES.LENDING]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.MEMO]: ALL_CLUSTERS,
  [PROGRAM_NAMES.MEMO_2]: ALL_CLUSTERS,
  [PROGRAM_NAMES.NAME]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.STAKE_POOL]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.SWAP]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.TOKEN]: ALL_CLUSTERS,
  [PROGRAM_NAMES.TOKEN_METADATA]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.TOKEN_VAULT]: LIVE_CLUSTERS,

  // other
  [PROGRAM_NAMES.ACUMEN]: MAINNET_ONLY,
  [PROGRAM_NAMES.BONFIDA_POOL]: MAINNET_ONLY,
  [PROGRAM_NAMES.BREAK_SAFECOIN]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.MANGO_GOVERNANCE]: MAINNET_ONLY,
  [PROGRAM_NAMES.MANGO_ICO]: MAINNET_ONLY,
  [PROGRAM_NAMES.MANGO_1]: MAINNET_ONLY,
  [PROGRAM_NAMES.MANGO_2]: MAINNET_ONLY,
  [PROGRAM_NAMES.MANGO_3]: MAINNET_ONLY,
  [PROGRAM_NAMES.MARINADE]: MAINNET_ONLY,
  [PROGRAM_NAMES.MERCURIAL]: [Cluster.Devnet, Cluster.MainnetBeta] as Cluster[],
  [PROGRAM_NAMES.METAPLEX]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.NFT_AUCTION]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.NFT_CANDY_MACHINE]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.ORCA_SWAP_1]: MAINNET_ONLY,
  [PROGRAM_NAMES.ORCA_SWAP_2]: MAINNET_ONLY,
  [PROGRAM_NAMES.ORCA_AQUAFARM]: MAINNET_ONLY,
  [PROGRAM_NAMES.PORT]: MAINNET_ONLY,
  [PROGRAM_NAMES.PYTH]: MAINNET_ONLY,
  [PROGRAM_NAMES.QUARRY_MERGE_MINE]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.QUARRY_MINE]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.QUARRY_MINT_WRAPPER]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.QUARRY_REDEEMER]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.QUARRY_REGISTRY]: LIVE_CLUSTERS,
  [PROGRAM_NAMES.RAYDIUM_AMM]: MAINNET_ONLY,
  [PROGRAM_NAMES.RAYDIUM_IDO]: MAINNET_ONLY,
  [PROGRAM_NAMES.RAYDIUM_LP_1]: MAINNET_ONLY,
  [PROGRAM_NAMES.RAYDIUM_LP_2]: MAINNET_ONLY,
  [PROGRAM_NAMES.RAYDIUM_STAKING]: MAINNET_ONLY,
  [PROGRAM_NAMES.SABER_ROUTER]: [
    Cluster.Devnet,
    Cluster.MainnetBeta,
  ] as Cluster[],
  [PROGRAM_NAMES.SABER_SWAP]: [
    Cluster.Devnet,
    Cluster.MainnetBeta,
  ] as Cluster[],
  [PROGRAM_NAMES.SERUM_1]: MAINNET_ONLY,
  [PROGRAM_NAMES.SERUM_2]: MAINNET_ONLY,
  [PROGRAM_NAMES.SERUM_3]: MAINNET_ONLY,
  [PROGRAM_NAMES.SERUM_SWAP]: MAINNET_ONLY,
  [PROGRAM_NAMES.SAFEEND]: MAINNET_ONLY,
  [PROGRAM_NAMES.SAFEIDO]: MAINNET_ONLY,
  [PROGRAM_NAMES.STEP_SWAP]: MAINNET_ONLY,
  [PROGRAM_NAMES.SWITCHBOARD]: MAINNET_ONLY,
  [PROGRAM_NAMES.WORMHOLE]: MAINNET_ONLY,
} as const;

export const PROGRAM_NAME_BY_ID = {
  // native built-ins
  AddressMap111111111111111111111111111111111: PROGRAM_NAMES.ADDRESS_MAP,
  Config1111111111111111111111111111111111111: PROGRAM_NAMES.CONFIG,
  [StakeProgram.programId.toBase58()]: PROGRAM_NAMES.STAKE,
  [SystemProgram.programId.toBase58()]: PROGRAM_NAMES.SYSTEM,
  [VOTE_PROGRAM_ID.toBase58()]: PROGRAM_NAMES.VOTE,
  [Secp256k1Program.programId.toBase58()]: PROGRAM_NAMES.SECP256K1,

  // spl
  AToD9iqHSc2fhEP9Jp7UYA6mRjHQ4CTWyzCsw8X3tH7K: PROGRAM_NAMES.ASSOCIATED_TOKEN,
  FEAj1Fwb2c9Kx9uHLGB2WH4Qhp2vACsJoudMVYHfE3ek: PROGRAM_NAMES.FEATURE_PROPOSAL,
  LENeX3L4CE1euBZp4zUNuicLP2SUZCbgXYZgBpZ9hWZ: PROGRAM_NAMES.LENDING,
  MEMDqRW2fYAU19mcFnoDVoqG4Br4t7TdyWjjv38P6Nc: PROGRAM_NAMES.MEMO,
  MEMWKbqsjEB8o972BvDHExZFSauzGZKvB4xHDVPFowh: PROGRAM_NAMES.MEMO_2,
  namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX: PROGRAM_NAMES.NAME,
  SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy: PROGRAM_NAMES.STAKE_POOL,
  SWPUnynS7FHA1koTbvmRktQgCDs7Tf4RkqwH19e2qSP: PROGRAM_NAMES.SWAP,
  ToKLx75MGim1d1jRusuVX8xvdvvbSDESVaNXpRA9PHN: PROGRAM_NAMES.TOKEN,
  metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s: PROGRAM_NAMES.TOKEN_METADATA,
  vau1zxA2LbssAUEF7Gpw91zMM1LvXrvpzJtmZ58rPsn: PROGRAM_NAMES.TOKEN_VAULT,

  // other
  C64kTdg1Hzv5KoQmZrQRcm2Qz7PkxtFBgw7EpFhvYn8W: PROGRAM_NAMES.ACUMEN,
  BoNd368PVJ4KLKRjUAzq3QXxeSjMqP3AxSwF8xK4FLa2: PROGRAM_NAMES.BONFIDA_POOL,
  BRKvqZwnXaukqUsUZK8MuDExAV3CLnMSmLwnGcahjWbk: PROGRAM_NAMES.BREAK_SAFECOIN,
  GqTPL6qRf5aUuqscLh8Rg2HTxPUXfhhAXDptTLhp1t2J: PROGRAM_NAMES.MANGO_GOVERNANCE,
  "7sPptkymzvayoSbLXzBsXEF8TSf3typNnAWkrKrDizNb": PROGRAM_NAMES.MANGO_ICO,
  JD3bq9hGdy38PuWQ4h2YJpELmHVGPPfFSuFkpzAd9zfu: PROGRAM_NAMES.MANGO_1,
  "5fNfvyp5czQVX77yoACa3JJVEhdRaWjPuazuWgjhTqEH": PROGRAM_NAMES.MANGO_2,
  mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68: PROGRAM_NAMES.MANGO_3,
  MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD: PROGRAM_NAMES.MARINADE,
  MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky: PROGRAM_NAMES.MERCURIAL,
  p1exdMJcjVao65QdewkaZRUnU6VPSXhus9n2GzWfh98: PROGRAM_NAMES.METAPLEX,
  auctxRXPeJoc4817jDhf4HbjnhEcr1cCXenosMhK5R8: PROGRAM_NAMES.NFT_AUCTION,
  cndyAnrLdpjq1Ssp1z8xxDsB8dxe7u4HL5Nxi2K5WXZ: PROGRAM_NAMES.NFT_CANDY_MACHINE,
  DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1: PROGRAM_NAMES.ORCA_SWAP_1,
  "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP": PROGRAM_NAMES.ORCA_SWAP_2,
  "82yxjeMsvaURa4MbZZ7WZZHfobirZYkH1zF8fmeGtyaQ": PROGRAM_NAMES.ORCA_AQUAFARM,
  Port7uDYB3wk6GJAw4KT1WpTeMtSu9bTcChBHkX2LfR: PROGRAM_NAMES.PORT,
  FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH: PROGRAM_NAMES.PYTH,
  QMMD16kjauP5knBwxNUJRZ1Z5o3deBuFrqVjBVmmqto: PROGRAM_NAMES.QUARRY_MERGE_MINE,
  QMNeHCGYnLVDn1icRAfQZpjPLBNkfGbSKRB83G5d8KB: PROGRAM_NAMES.QUARRY_MINE,
  QMWoBmAyJLAsA1Lh9ugMTw2gciTihncciphzdNzdZYV:
    PROGRAM_NAMES.QUARRY_MINT_WRAPPER,
  QRDxhMw1P2NEfiw5mYXG79bwfgHTdasY2xNP76XSea9: PROGRAM_NAMES.QUARRY_REDEEMER,
  QREGBnEj9Sa5uR91AV8u3FxThgP5ZCvdZUW2bHAkfNc: PROGRAM_NAMES.QUARRY_REGISTRY,
  "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8": PROGRAM_NAMES.RAYDIUM_AMM,
  "9HzJyW1qZsEiSfMUf6L2jo3CcTKAyBmSyKdwQeYisHrC": PROGRAM_NAMES.RAYDIUM_IDO,
  RAYFrn8s4Bsx7NY6gXQPpnWax5dDgd3MCayubNVcZvd: PROGRAM_NAMES.RAYDIUM_LP_1,
  "RAYFZ91CAQSREwhzPVssQM6w33uYPfKM5Nea6o4mH1t": PROGRAM_NAMES.RAYDIUM_LP_2,
  RAYqdf1KnhLXkfvhvxQXajuRY2Gq3pHvv3Ucr7WwM6t: PROGRAM_NAMES.RAYDIUM_STAKING,
  Crt7UoUR6QgrFrN7j8rmSQpUTNWNSitSwWvsWGf1qZ5t: PROGRAM_NAMES.SABER_ROUTER,
  SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ: PROGRAM_NAMES.SABER_SWAP,
  SRMGrk9ZtEfUfh6vNYGxbNsZULnXSiAHx9SPvMbY2mU: PROGRAM_NAMES.SERUM_1,
  SRMijGeYEx7F7Wjq8kpwCUx9zuUECye3wyk3oGwqpXu: PROGRAM_NAMES.SERUM_2,
  "SRMrEgnzRgGMQ8QzcL8cjWr5xpdVs1KQCQ58Jkkq1qx": PROGRAM_NAMES.SERUM_3,
  "22Y43yTVxuUkoRKdm9thyRhQ3SdgQS7c7kB6UNCiaczD": PROGRAM_NAMES.SERUM_SWAP,
  So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo: PROGRAM_NAMES.SAFEEND,
  CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi: PROGRAM_NAMES.SAFEIDO,
  SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1: PROGRAM_NAMES.STEP_SWAP,
  DtmE9D2CSB4L5D6A15mraeEjrGMm6auWVzgaD8hK2tZM: PROGRAM_NAMES.SWITCHBOARD,
  WRMYas5GNR2R6YJjHSbkttRghwxf3hYMhZWnhRdfXXy: PROGRAM_NAMES.WORMHOLE,
} as const;

export type LoaderName = typeof LOADER_IDS[keyof typeof LOADER_IDS];
export const LOADER_IDS = {
  MoveLdr111111111111111111111111111111111111: "Move Loader",
  NativeLoader1111111111111111111111111111111: "Native Loader",
  [BPF_LOADER_DEPRECATED_PROGRAM_ID.toBase58()]: "BPF Loader",
  [BPF_LOADER_PROGRAM_ID.toBase58()]: "BPF Loader 2",
  BPFLoaderUpgradeab1e11111111111111111111111: "BPF Upgradeable Loader",
} as const;

export const SPECIAL_IDS: { [key: string]: string } = {
  "1nc1nerator11111111111111111111111111111111": "Incinerator",
  Sysvar1111111111111111111111111111111111111: "SYSVAR",
};

export const SYSVAR_IDS = {
  [SYSVAR_CLOCK_PUBKEY.toBase58()]: "Sysvar: Clock",
  SysvarEpochSchedu1e111111111111111111111111: "Sysvar: Epoch Schedule",
  SysvarFees111111111111111111111111111111111: "Sysvar: Fees",
  SysvarRecentB1ockHashes11111111111111111111: "Sysvar: Recent Blockhashes",
  [SYSVAR_RENT_PUBKEY.toBase58()]: "Sysvar: Rent",
  [SYSVAR_REWARDS_PUBKEY.toBase58()]: "Sysvar: Rewards",
  SysvarS1otHashes111111111111111111111111111: "Sysvar: Slot Hashes",
  SysvarS1otHistory11111111111111111111111111: "Sysvar: Slot History",
  [SYSVAR_STAKE_HISTORY_PUBKEY.toBase58()]: "Sysvar: Stake History",
  Sysvar1nstructions1111111111111111111111111: "Sysvar: Instructions",
};

export function programLabel(
  address: string,
  cluster: Cluster
): string | undefined {
  const programName = PROGRAM_NAME_BY_ID[address];
  if (programName && PROGRAM_DEPLOYMENTS[programName].includes(cluster)) {
    return programName;
  }

  return LOADER_IDS[address];
}

export function tokenLabel(
  address: string,
  tokenRegistry?: TokenInfoMap
): string | undefined {
  if (!tokenRegistry) return;
  const tokenInfo = tokenRegistry.get(address);
  if (!tokenInfo) return;
  if (tokenInfo.name === tokenInfo.symbol) {
    return tokenInfo.name;
  }
  return `${tokenInfo.symbol} - ${tokenInfo.name}`;
}

export function addressLabel(
  address: string,
  cluster: Cluster,
  tokenRegistry?: TokenInfoMap
): string | undefined {
  return (
    programLabel(address, cluster) ||
    SYSVAR_IDS[address] ||
    SPECIAL_IDS[address] ||
    tokenLabel(address, tokenRegistry) ||
    SerumMarketRegistry.get(address, cluster)
  );
}

export function displayAddress(
  address: string,
  cluster: Cluster,
  tokenRegistry: TokenInfoMap
): string {
  return addressLabel(address, cluster, tokenRegistry) || address;
}

export function intoTransactionInstruction(
  tx: ParsedTransaction,
  instruction: ParsedInstruction | PartiallyDecodedInstruction
): TransactionInstruction | undefined {
  const message = tx.message;
  if ("parsed" in instruction) return;

  const keys = [];
  for (const account of instruction.accounts) {
    const accountKey = message.accountKeys.find(({ pubkey }) =>
      pubkey.equals(account)
    );
    if (!accountKey) return;
    keys.push({
      pubkey: accountKey.pubkey,
      isSigner: accountKey.signer,
      isWritable: accountKey.writable,
    });
  }

  return new TransactionInstruction({
    data: bs58.decode(instruction.data),
    keys: keys,
    programId: instruction.programId,
  });
}

export function intoParsedTransaction(tx: Transaction): ParsedTransaction {
  const message = tx.compileMessage();
  return {
    signatures: tx.signatures.map((value) =>
      bs58.encode(value.signature as any)
    ),
    message: {
      accountKeys: message.accountKeys.map((key, index) => ({
        pubkey: key,
        signer: tx.signatures.some(({ publicKey }) => publicKey.equals(key)),
        writable: message.isAccountWritable(index),
      })),
      instructions: message.instructions.map((ix) => ({
        programId: message.accountKeys[ix.programIdIndex],
        accounts: ix.accounts.map((index) => message.accountKeys[index]),
        data: ix.data,
      })),
      recentBlockhash: message.recentBlockhash,
    },
  };
}
