const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log('starting test');

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DefiApp;
  const baseAccount = anchor.web3.Keypair.generate();
  const tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log('transaction signature', tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);

  await program.rpc.deposit('500000', {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('Account after first deposit', account);

  // await program.rpc.deposit('5000', {
  //   accounts: {
  //     baseAccount: baseAccount.publicKey,
  //     user: provider.wallet.publicKey,
  //   },
  // });

  // account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  // console.log('Account after second deposit', account);

  await program.rpc.borrow('250000', {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('Account after borrow', account);
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();
