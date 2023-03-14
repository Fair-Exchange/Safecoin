import React, { useMemo } from "react";
import { Account } from "providers/accounts";
import { useCluster } from "providers/cluster";
import { BorshAccountsCoder } from "@safecoin/anchor";
import { IdlTypeDef } from "@safecoin/anchor/dist/cjs/idl";
import { getAnchorProgramName, mapAccountToRows } from "utils/anchor";
import { ErrorCard } from "components/common/ErrorCard";
//import { useAnchorProgram } from "providers/anchor";

export function AnchorAccountCard({ account }: { account: Account }) {
  const { lamports } = account;
  const { url } = useCluster();
  const anchorProgram = "ancAHhi4TqE5nwR29gPGhGV8CYLgHUFEDfE1q12nRK3";//useAnchorProgram(    account.details?.owner.toString() || "",    url  );
  const rawData = account?.details?.rawData;
  const programName = "Unknown Program";

  const decodedAccountData = '';
  const accountDef = '';
 

  if (lamports === undefined) return null;
  if (!anchorProgram) return <ErrorCard text="No Anchor IDL found" />;
  if (!decodedAccountData || !accountDef) {
    return (
      <ErrorCard text="Failed to decode account data according to the public Anchor interface" />
    );
  }

  return (
    <div>
      <div className="card">
        <div className="card-header">
          <div className="row align-items-center">
            <div className="col">
              <h3 className="card-header-title">
                {programName}: "program"
              </h3>
            </div>
          </div>
        </div>

        <div className="table-responsive mb-0">
          <table className="table table-sm table-nowrap card-table">
            <thead>
              <tr>
                <th className="w-1">Field</th>
                <th className="w-1">Type</th>
                <th className="w-1">Value</th>
              </tr>
            </thead>
            <tbody>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
