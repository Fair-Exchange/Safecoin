import * as web3 from '@safecoin/web3.js';

(async () => {
  // Connect to cluster
  var connection = new web3.Connection(
    web3.clusterApiUrl('devnet'),
    'confirmed',
  );

  // Generate a new wallet keypair and airdrop SAFE
  var wallet = web3.Keypair.generate();
  var airdropSignature = await connection.requestAirdrop(
    wallet.publicKey,
    web3.LAMPORTS_PER_SAFE,
  );

  //wait for airdrop confirmation
  await connection.confirmTransaction(airdropSignature);

  // get account info
  // account data is bytecode that needs to be deserialized
  // serialization and deserialization is program specific
  let account = await connection.getAccountInfo(wallet.publicKey);
  console.log(account);
})();
