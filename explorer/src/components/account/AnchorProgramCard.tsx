import { PublicKey } from "@safecoin/web3.js";
//import { useAnchorProgram } from "providers/anchor";
import { useCluster } from "providers/cluster";
import ReactJson from "react-json-view";

export function AnchorProgramCard({ programId }: { programId: PublicKey }) {
  const { url } = useCluster();
  const program = "ancAHhi4TqE5nwR29gPGhGV8CYLgHUFEDfE1q12nRK3";   //useAnchorProgram(programId.toString(), url);

  if (!program) {
    return null;
  }

  return (
    <>
      <div className="card">
        <div className="card-header">
          <div className="row align-items-center">
            <div className="col">
              <h3 className="card-header-title">Anchor IDL</h3>
            </div>
          </div>
        </div>


      </div>
    </>
  );
}
