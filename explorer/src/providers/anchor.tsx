import { Idl, Program } from "@safecoin/anchor";
import { Connection, Keypair } from "@safecoin/web3.js";
//import { NodeWallet } from "@safecoin/js";

const Provider = '';
const Nodewallet = '';

const cachedAnchorProgramPromises: Record<
  string,
  | void
  | { __type: "promise"; promise: Promise<void> }
  | { __type: "result"; result: Program<Idl> | null }
> = {};



export type AnchorAccount = {
  layout: string;
  account: Object;
};