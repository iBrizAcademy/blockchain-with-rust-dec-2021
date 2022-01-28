import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';

import {getPayer, getRpcUrl, createKeypairFromFile} from './utils';


let connection: Connection;
let payer: Keypair;
let programId: PublicKey;
let quotesPubKey: PublicKey;
const PROGRAM_PATH = path.resolve(__dirname, '../quotes_solana/dist/program');
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'quotes_solana.so');

const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'quotes_solana-keypair.json');

class QuoteAccount {
    quote = '';
    counter = 0;
    constructor(fields: {quote: string, counter: number} | undefined = undefined) {
        if (fields) {
            this.quote = fields.quote;
            this.counter = fields.counter;
        }
    }
}

const QuoteSchema = new Map([
    [QuoteAccount, {kind: 'struct', fields: [['quote', 'String'],['counter', 'u32']]}],
]);

const QUOTES_SIZE = borsh.serialize(
    QuoteSchema,
    new QuoteAccount(),
).length;

export async function establishConnection(): Promise<void> {
    const rpcUrl = await getRpcUrl();
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
}



export async function establishPayer(): Promise<void> {
    let fees = 0;
    if (!payer) {
        const {feeCalculator} = await connection.getRecentBlockhash();

        // Calculate the cost to fund the greeter account
        fees += await connection.getMinimumBalanceForRentExemption(QUOTES_SIZE);

        // Calculate the cost of sending transactions
        fees += feeCalculator.lamportsPerSignature * 100; // wag

        payer = await getPayer();
    }

    let lamports = await connection.getBalance(payer.publicKey);
    if (lamports < fees) {
        // If current balance is not enough to pay for fees, request an airdrop
        const sig = await connection.requestAirdrop(
            payer.publicKey,
            fees - lamports,
        );
        await connection.confirmTransaction(sig);
        lamports = await connection.getBalance(payer.publicKey);
    }

    console.log(
        'Using account',
        payer.publicKey.toBase58(),
        'containing',
        lamports / LAMPORTS_PER_SOL,
        'SOL to pay for fees',
    );
}


export async function checkProgram(): Promise<void> {
    // Read program id from keypair file
    try {
        const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
        programId = programKeypair.publicKey;
    } catch (err) {
        const errMsg = (err as Error).message;
        throw new Error(
            `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
        );
    }

    // Check if the program has been deployed
    const programInfo = await connection.getAccountInfo(programId);
    if (programInfo === null) {
        if (fs.existsSync(PROGRAM_SO_PATH)) {
            throw new Error(
                'Program needs to be deployed with `solana program deploy ./dist/program/quotes_solana.so`',
            );
        } else {
            throw new Error('Program needs to be built and deployed');
        }
    } else if (!programInfo.executable) {
        throw new Error(`Program is not executable`);
    }
    console.log(`Using program ${programId.toBase58()}`);

    // Derive the address (public key) of a quote account from the program so that it's easy to find later.
    const QuoteSeed = 'quote';
    quotesPubKey = await PublicKey.createWithSeed(
        payer.publicKey,
        QuoteSeed,
        programId,
    );

    const quoteAccount = await connection.getAccountInfo(quotesPubKey);
    if (quoteAccount === null) {
        console.log(
            'Creating account',
            quotesPubKey.toBase58(),
            'to say hello to',
        );
        const lamports = await connection.getMinimumBalanceForRentExemption(
            QUOTES_SIZE,
        );

        const transaction = new Transaction().add(
            SystemProgram.createAccountWithSeed({
                fromPubkey: payer.publicKey,
                basePubkey: payer.publicKey,
                seed: QuoteSeed,
                newAccountPubkey: quotesPubKey,
                lamports,
                space: QUOTES_SIZE,
                programId,
            }),
        );
        await sendAndConfirmTransaction(connection, transaction, [payer]);
    }
}


export async function setQuote(): Promise<void> {
    console.log('Saying hello to', quotesPubKey.toBase58());
    const instruction = new TransactionInstruction({
        keys: [{pubkey: quotesPubKey, isSigner: false, isWritable: true}],
        programId,
        data: Buffer.alloc(0), // All instructions are hellos
    });
    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer],
    );
}


export async function getQuote(): Promise<void> {
    const accountInfo = await connection.getAccountInfo(quotesPubKey);
    if (accountInfo === null) {
        throw 'Error: cannot find the quotes account';
    }
    const latestQuote = borsh.deserialize(
        QuoteSchema,
        QuoteAccount,
        accountInfo.data,
    );
    console.log('Qoute Public Key:', quotesPubKey.toBase58())
    console.log('Latest quote:', latestQuote.quote);
    console.log('Quote Counter:', latestQuote.counter);

}