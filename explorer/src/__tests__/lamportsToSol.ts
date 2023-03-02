import { expect } from "chai";
import { lamportsToSafe, LAMPORTS_PER_SAFE } from "utils";

describe("lamportsToSafe", () => {
  it("0 lamports", () => {
    expect(lamportsToSafe(0)).to.eq(0.0);
    expect(lamportsToSafe(BigInt(0))).to.eq(0.0);
  });

  it("1 lamport", () => {
    expect(lamportsToSafe(1)).to.eq(0.000000001);
    expect(lamportsToSafe(BigInt(1))).to.eq(0.000000001);
    expect(lamportsToSafe(-1)).to.eq(-0.000000001);
    expect(lamportsToSafe(BigInt(-1))).to.eq(-0.000000001);
  });

  it("1 SAFE", () => {
    expect(lamportsToSafe(LAMPORTS_PER_SAFE)).to.eq(1.0);
    expect(lamportsToSafe(BigInt(LAMPORTS_PER_SAFE))).to.eq(1.0);
    expect(lamportsToSafe(-LAMPORTS_PER_SAFE)).to.eq(-1.0);
    expect(lamportsToSafe(BigInt(-LAMPORTS_PER_SAFE))).to.eq(-1.0);
  });

  it("u64::MAX lamports", () => {
    expect(lamportsToSafe(2n ** 64n)).to.eq(18446744073.709551615);
    expect(lamportsToSafe(-(2n ** 64n))).to.eq(-18446744073.709551615);
  });
});
