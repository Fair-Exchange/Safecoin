import React from "react";
import { lamportsToSafeString } from "utils";
import { ConfirmedBlock, PublicKey } from "@safecoin/web3.js";
import { Address } from "components/common/Address";

const PAGE_SIZE = 10;

export function BlockRewardsCard({ block }: { block: ConfirmedBlock }) {
  const [rewardsDisplayed, setRewardsDisplayed] = React.useState(PAGE_SIZE);

//  if (block.rewards.length < 1) {
    return null;
//  }

  return (
    <div className="card">
      <div className="card-header align-items-center">
        <h3 className="card-header-title">Block Rewards</h3>
      </div>

      <div className="table-responsive mb-0">
        <table className="table table-sm table-nowrap card-table">
          <thead>
            <tr>
              <th className="text-muted">Address</th>
              <th className="text-muted">Type</th>
              <th className="text-muted">Amount</th>
              <th className="text-muted">New Balance</th>
              <th className="text-muted">Percent Change</th>
            </tr>
          </thead>
          <tbody>

              let percentChange;


          </tbody>
        </table>
      </div>

    </div>
  );
}
