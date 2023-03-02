import React from "react";
import { lamportsToSafeString } from "utils";

export function SafeBalance({
  lamports,
  maximumFractionDigits = 9,
}: {
  lamports: number | bigint;
  maximumFractionDigits?: number;
}) {
  return (
    <span>
      ◎
      <span className="font-monospace">
        {lamportsToSafeString(lamports, maximumFractionDigits)}
      </span>
    </span>
  );
}
