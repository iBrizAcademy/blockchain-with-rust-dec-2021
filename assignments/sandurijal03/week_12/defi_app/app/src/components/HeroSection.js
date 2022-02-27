import { useEffect, useState } from 'react';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { Program, Provider, web3 } from '@project-serum/anchor';
import { Buffer } from 'buffer';

import kp from '../keypair.json';
import idl from '../idl.json';
import {
  BorrowButton,
  DepositButton,
  Input,
  InputContainer,
  Bottom,
  Title,
  Description,
  Top,
  Wrapper,
  Screen,
  Button,
} from './styles';

window.Buffer = Buffer;

const { SystemProgram } = web3;
const arr = Object.values(kp._keypair.secretKey);
const secret = new Uint8Array(arr);
const baseAccount = web3.Keypair.fromSecretKey(secret);

const programID = new PublicKey(idl.metadata.address);

const network = clusterApiUrl('devnet');

const opts = {
  preflightCommitment: 'processed',
};

export const HeroSection = () => {
  const [walletAddress, setWalletAddress] = useState(null);
  const [borrowAmount, setBorrowAmount] = useState('');
  const [depositAmount, setDepositAmount] = useState('');
  const [amount, setAmount] = useState(0);
  const [transaction, setTransaction] = useState([]);

  const getProvider = () => {
    const connection = new Connection(network, opts.preflightCommitment);
    const provider = new Provider(
      connection,
      window.solana,
      opts.preflightCommitment,
    );
    return provider;
  };

  const checkIfWalletIsConnected = async () => {
    try {
      const { solana } = window;
      if (solana) {
        if (solana.isPhantom) {
          const response = await solana.connect({ onlyIfTrusted: true });
          setWalletAddress(response.publicKey.toString());
        }
      } else {
        alert('Solana object not found. Get a phantom wallet.');
      }
    } catch (error) {
      console.error(error);
    }
  };

  const createMarketplaceAccount = async () => {
    try {
      const provider = getProvider();
      const program = new Program(idl, programID, provider);
      await program.rpc.initialize({
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount],
      });
      console.log(
        'created a new base acoound ',
        baseAccount.publicKey.toString(),
      );
      await getTransaction();
    } catch (error) {
      console.error('Error creating base account', error);
    }
  };

  useEffect(() => {
    const onLoad = async () => {
      await checkIfWalletIsConnected();
    };
    window.addEventListener('load', onLoad);
    return () => window.removeEventListener('load', onLoad);
  }, []);

  const getTransaction = async () => {
    try {
      const provider = getProvider();
      const program = new Program(idl, programID, provider);
      const account = await program.account.baseAccount.fetch(
        baseAccount.publicKey,
      );
      setAmount(account.totalBalance.words[0]);
      setTransaction(account.tokens);
    } catch (error) {
      console.error('error in getting transaction', error);
      setTransaction(null);
    }
  };

  const sendToken = async () => {
    if (depositAmount <= 0) {
      console.log('cannot deposit amount in negative');
      return;
    }
    setDepositAmount('');

    try {
      const provider = getProvider();
      const program = new Program(idl, programID, provider);
      await program.rpc.deposit(depositAmount, {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
        },
      });
      const account = await program.account.baseAccount.fetch(
        baseAccount.publicKey,
      );
      console.log(account);
      console.log('deposit successful', depositAmount);
      await getTransaction();
    } catch (error) {
      console.error('error in sending token', error);
    }
  };

  const borrowToken = async () => {
    if (borrowAmount <= 0) {
      console.log('cannot borrow amount in negative');
      return;
    }
    setBorrowAmount('');

    try {
      const provider = getProvider();
      const program = new Program(idl, programID, provider);
      await program.rpc.borrow(borrowAmount, {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
        },
      });
      const account = await program.account.baseAccount.fetch(
        baseAccount.publicKey,
      );
      // console.log('borrow successful', account.message);
      console.log(account);
      await getTransaction();
    } catch (error) {
      console.error('error in sending token', error);
    }
  };

  useEffect(() => {
    if (walletAddress) {
      getTransaction();
    }
  }, [walletAddress]);

  const onDepositChange = (e) => {
    setDepositAmount(e.target.value);
  };

  const onBorrowChange = (e) => {
    setBorrowAmount(e.target.value);
  };

  const renderConnectedContainer = () => {
    if (transaction === null) {
      return (
        <Button onClick={createMarketplaceAccount}>Connect to wallet</Button>
      );
    } else {
      return (
        <Bottom>
          <InputContainer>
            <Screen>{amount}</Screen>
          </InputContainer>

          <InputContainer>
            <Input
              type='text'
              value={depositAmount}
              onChange={onDepositChange}
            />
            <DepositButton onClick={sendToken}>deposit</DepositButton>
          </InputContainer>

          <InputContainer>
            <Input type='text' value={borrowAmount} onChange={onBorrowChange} />
            <BorrowButton onClick={borrowToken}>borrow</BorrowButton>
          </InputContainer>
        </Bottom>
      );
    }
  };

  return (
    <Wrapper>
      <Top>
        <Title>Solana Marketplace</Title>
        <Description>Digital wallet with metaverse</Description>
        {!walletAddress && (
          <Button onClick={createMarketplaceAccount}>Connect to wallet</Button>
        )}
      </Top>
      {walletAddress && renderConnectedContainer()}
    </Wrapper>
  );
};
