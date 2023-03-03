import React from "react";
import { BigNumber } from "bignumber.js";
import { SafeBalance } from "components/common/SafeBalance";

export function BalanceDelta({
  delta,
  isSafe = false,
}: {
  delta: BigNumber;
  isSafe?: boolean;
}) {
  let sols;

  if (isSafe) {
    sols = <SafeBalance lamports={Math.abs(delta.toNumber())} />;
  }

  if (delta.gt(0)) {
    return (
      <span className="badge bg-success-soft">
        +{isSafe ? sols : delta.toString()}
      </span>
    );
  } else if (delta.lt(0)) {
    return (
      <span className="badge bg-warning-soft">
        {isSafe ? <>-{sols}</> : delta.toString()}
      </span>
    );
  }

  return <span className="badge bg-secondary-soft">0</span>;
}
