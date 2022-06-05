import React from "react";
import {
  SignatureResult,
  StakeProgram,
  ParsedInstruction,
} from "@safecoin/web3.js";
import { SafeBalance } from "utils";
import { InstructionCard } from "../InstructionCard";
import { Address } from "components/common/Address";
import { SplitInfo } from "./types";

export function SplitDetailsCard(props: {
  ix: ParsedInstruction;
  index: number;
  result: SignatureResult;
  info: SplitInfo;
  innerCards?: JSX.Element[];
  childIndex?: number;
}) {
  const { ix, index, result, info, innerCards, childIndex } = props;

  return (
    <InstructionCard
      ix={ix}
      index={index}
      result={result}
      title="Stake Program: Split Stake"
      innerCards={innerCards}
      childIndex={childIndex}
    >
      <tr>
        <td>Program</td>
        <td className="text-lg-end">
          <Address pubkey={StakeProgram.programId} alignRight link />
        </td>
      </tr>

      <tr>
        <td>Stake Address</td>
        <td className="text-lg-end">
          <Address pubkey={info.stakeAccount} alignRight link />
        </td>
      </tr>

      <tr>
        <td>Authority Address</td>
        <td className="text-lg-end">
          <Address pubkey={info.stakeAuthority} alignRight link />
        </td>
      </tr>

      <tr>
        <td>New Stake Address</td>
        <td className="text-lg-end">
          <Address pubkey={info.newSplitAccount} alignRight link />
        </td>
      </tr>

      <tr>
        <td>Split Amount (SAFE)</td>
        <td className="text-lg-end">
          <SafeBalance lamports={info.lamports} />
        </td>
      </tr>
    </InstructionCard>
  );
}
