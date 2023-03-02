import React from "react";
import { Link } from "react-router-dom";

type FlaggedMap = Record<string, IncidentDescription>;

type IncidentId = "ftx-hack-november-2022" | "known-scam";
type IncidentDescription = React.ReactElement;

const FLAGGED_ACCOUNTS: Record<string, IncidentId> = {
  GACpXND1SSfTSQMmqGuFvGwXB3jGEYBDRGNzmLfTYwSP: "known-scam",
  "9tAViia54YAaL9gv92hBu8K4QGRBKbytCQ9TYsJ6F6or": "known-scam",
  // Serum Swap
  "22Y43yTVxuUkoRKdm9thyRhQ3SdgQS7c7kB6UNCiaczD": "ftx-hack-november-2022",
  // Serum Dex V3
  "SRMrEgnzRgGMQ8QzcL8cjWr5xpdVs1KQCQ58Jkkq1qx": "ftx-hack-november-2022",
  // Serum Dex V2
  SRMijGeYEx7F7Wjq8kpwCUx9zuUECye3wyk3oGwqpXu: "ftx-hack-november-2022",
  // Serum Dex V1
  SRMGrk9ZtEfUfh6vNYGxbNsZULnXSiAHx9SPvMbY2mU: "ftx-hack-november-2022",
};

const INCIDENTS: Record<IncidentId, IncidentDescription> = {
  "known-scam": (
    <>
      <div className="alert alert-danger alert-scam" role="alert">
        Warning! This account has been flagged by the community as a scam
        account. Please be cautious sending SAFE to this account.
      </div>
    </>
  ),
  "ftx-hack-november-2022": (
    <>
      <div className="alert alert-danger alert-scam" role="alert">
        Warning! This program's upgrade key may have been compromised by the FTX
        hack. Please migrate to the community fork:{" "}
        <Link
          className="text-white"
          style={{ textDecoration: "underline" }}
          to="https://github.com/openbook-dex/program"
        >
          https://github.com/openbook-dex/program
        </Link>
      </div>
    </>
  ),
} as const;

const FLAGGED_ACCOUNTS_WARNING: FlaggedMap = {};
for (const [account, incidentId] of Object.entries(FLAGGED_ACCOUNTS)) {
  FLAGGED_ACCOUNTS_WARNING[account] = INCIDENTS[incidentId];
}
export default FLAGGED_ACCOUNTS_WARNING;
