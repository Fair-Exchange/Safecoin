import React from "react";
import "bootstrap/dist/js/bootstrap.min.js";
import {
  NFTData,
  useFetchAccountInfo,
  useMintAccountInfo,
} from "providers/accounts";
//import { programs } from "@safecoin/js";
import { ArtContent } from "components/common/NFTArt";
import { InfoTooltip } from "components/common/InfoTooltip";
import { clusterPath } from "utils/url";
import { Link } from "react-router-dom";
//import { EditionInfo } from "providers/accounts/utils/getEditionInfo";
import { PublicKey } from "@safecoin/web3.js";

const programs = '';
const EditionInfo = '';

export function NFTHeader({
  nftData,
  address,
}: {
  nftData: NFTData;
  address: string;
}) {
  const collectionAddress = '';//nftData.metadata.collection?.key;
  const collectionMintInfo = useMintAccountInfo(collectionAddress);
  const fetchAccountInfo = useFetchAccountInfo();

  React.useEffect(() => {
    if (collectionAddress && !collectionMintInfo) {
      fetchAccountInfo(new PublicKey(collectionAddress));
    }
  }, [fetchAccountInfo, collectionAddress]); // eslint-disable-line react-hooks/exhaustive-deps

  const metadata = nftData.metadata;
  const data = nftData.json;
  const isVerifiedCollection = '';
//    metadata.collection != null &&
//    metadata.collection?.verified &&
//    collectionMintInfo !== undefined;
//false
//  return (
//    <div className="row">
//      <div className="col-auto ms-2 d-flex align-items-center">
//        <ArtContent metadata={metadata} pubkey={address} data={data} />
//      </div>
//      <div className="col mb-3 ms-0.5 mt-3">
//        {<h6 className="header-pretitle ms-1">Metaplex NFT</h6>}


//            {metadata.data.name !== ""
//              ? metadata.data.name
//              "No NFT name was found"}

//          {getEditionPill(nftData.editionInfo)}
//          {isVerifiedCollection ? getVerifiedCollectionPill() : null}




//    </div>
//  );
}

type Creator = ''; //  programs.metadata.Creator;
function getCreatorDropdownItems(creators: Creator[] | null) {
  const CreatorHeader = () => {
    const creatorTooltip =
      "Verified creators signed the metadata associated with this NFT when it was created.";

    const shareTooltip =
      "The percentage of the proceeds a creator receives when this NFT is sold.";

    return (
      <div
        className={
          "d-flex align-items-center dropdown-header creator-dropdown-entry"
        }
      >
        <div className="d-flex font-monospace creator-dropdown-header">
          <span>Creator Address</span>
          <InfoTooltip bottom text={creatorTooltip} />
        </div>
        <div className="d-flex font-monospace">
          <span className="font-monospace">Royalty</span>
          <InfoTooltip bottom text={shareTooltip} />
        </div>
      </div>
    );
  };

  const getVerifiedIcon = (isVerified: boolean) => {
    const className = isVerified ? "fe fe-check" : "fe fe-alert-octagon";
    return <i className={`ms-3 ${className}`}></i>;
  };

  const CreatorEntry = (creator: Creator) => {
    return (
      <div
        className={
          "d-flex align-items-center font-monospace creator-dropdown-entry ms-3 me-3"
        }

        >


      </div>
    );
  };

  if (creators && creators.length > 0) {
    let listOfCreators: JSX.Element[] = [];

    listOfCreators.push(<CreatorHeader key={"header"} />);
    creators.forEach((creator) => {
//      listOfCreators.push(<CreatorEntry key={creator.address} {...creator} />);
    });

    return listOfCreators;
  }

  return (
    <div className={"dropdown-item font-monospace"}>
      <div className="me-3">No creators are associated with this NFT.</div>
    </div>
  );
}

function getEditionPill(editionInfo: '') {
  const masterEdition = '';//editionInfo.masterEdition;
  const edition = '';//editionInfo.edition;

  return (
''
  );
}

function getSaleTypePill(hasPrimarySaleHappened: boolean) {
  const primaryMarketTooltip =
    "Creator(s) split 100% of the proceeds when this NFT is sold.";

  const secondaryMarketTooltip =
    "Creator(s) split the Seller Fee when this NFT is sold. The owner receives the remaining proceeds.";

  return (
    <div className={"d-inline-flex align-items-center"}>
      <span className="badge badge-pill bg-dark">{`${
        hasPrimarySaleHappened ? "Secondary Market" : "Primary Market"
      }`}</span>
      <InfoTooltip
        bottom
        text={
          hasPrimarySaleHappened ? secondaryMarketTooltip : primaryMarketTooltip
        }
      />
    </div>
  );
}

function getIsMutablePill(isMutable: boolean) {
  return (
    <span className="badge badge-pill bg-dark">{`${
      isMutable ? "Mutable" : "Immutable"
    }`}</span>
  );
}

function getVerifiedCollectionPill() {
  const onchainVerifiedToolTip =
    "This NFT has been verified as a member of an on-chain collection. This tag guarantees authenticity.";
  return (
    <div className={"d-inline-flex align-items-center ms-2"}>
      <span className="badge badge-pill bg-dark">{"Verified Collection"}</span>
      <InfoTooltip bottom text={onchainVerifiedToolTip} />
    </div>
  );
}
