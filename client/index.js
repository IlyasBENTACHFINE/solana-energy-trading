const {
  Connection,
  PublicKey,
  Keypair,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL,
} = require('@solana/web3.js');
const borsh = require('@project-serum/borsh');
const readline = require('readline');

// Replace with your program ID
const PROGRAM_ID = new PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS');

// Connect to the Solana devnet
const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

// Generate a new keypair for testing
const payer = Keypair.generate();
console.log('Generated new keypair. Public key:', payer.publicKey.toBase58());

const serializeInstruction = (instruction) => {
  const layout = borsh.struct([
    borsh.u8('variant'),
    borsh.u8('participant_type'),
    borsh.u64('energy_amount'),
    borsh.u64('price'),
    borsh.u64('price_limit'),
    borsh.u64('amount'),
  ]);

  const buffer = Buffer.alloc(1000);
  const len = layout.encode(
    {
      variant: instruction.variant,
      participant_type: instruction.participant_type || 0,
      energy_amount: instruction.energy_amount || BigInt(0),
      price: instruction.price || BigInt(0),
      price_limit: instruction.price_limit || BigInt(0),
      amount: instruction.amount || BigInt(0),
    },
    buffer
  );
  return buffer.slice(0, len);
};

const sendInstruction = async (instruction) => {
  const transaction = new Transaction().add(
    new TransactionInstruction({
      keys: [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      ],
      programId: PROGRAM_ID,
      data: serializeInstruction(instruction),
    })
  );

  try {
    const signature = await sendAndConfirmTransaction(
      connection,
      transaction,
      [payer]
    );
    console.log('Transaction signature:', signature);
  } catch (error) {
    console.error('Error sending transaction:', error);
  }
};

const initializeLedger = () => {
  return sendInstruction({ variant: 0 });
};

const registerParticipant = (participantType) => {
  return sendInstruction({ variant: 1, participant_type: participantType });
};

const reportProduction = (energyAmount, price) => {
  return sendInstruction({ variant: 2, energy_amount: energyAmount, price });
};

const postDemand = (energyAmount, priceLimit) => {
  return sendInstruction({ variant: 3, energy_amount: energyAmount, price_limit: priceLimit });
};

const matchTransactions = () => {
  return sendInstruction({ variant: 4 });
};

const deposit = (amount) => {
  return sendInstruction({ variant: 5, amount });
};

const withdraw = (amount) => {
  return sendInstruction({ variant: 6, amount });
};

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

const question = (query) => new Promise((resolve) => rl.question(query, resolve));

const main = async () => {
  console.log('Checking account balance...');
  let balance = await connection.getBalance(payer.publicKey);
  console.log(`Current balance: ${balance / LAMPORTS_PER_SOL} SOL`);

  while (balance < 0.5 * LAMPORTS_PER_SOL) {
    console.log('\nInsufficient balance to perform transactions.');
    console.log('Please fund this address manually:');
    console.log(payer.publicKey.toBase58());
    console.log('\nYou can use the Solana CLI:');
    console.log(`solana airdrop 1 ${payer.publicKey.toBase58()} --url https://api.devnet.solana.com`);
    console.log('\nOr visit https://solfaucet.com/');
    
    await question('\nPress Enter after funding the account...');
    
    balance = await connection.getBalance(payer.publicKey);
    console.log(`Updated balance: ${balance / LAMPORTS_PER_SOL} SOL`);
  }

  console.log('Proceeding with transactions...');

  try {
    await initializeLedger();
    await registerParticipant(0); // 0 for Producer, 1 for Consumer, 2 for Prosumer
    await reportProduction(BigInt(1000), BigInt(50)); // 1000 units at 50 tokens each
    await postDemand(BigInt(500), BigInt(60)); // Demand for 500 units with a max price of 60 tokens
    await matchTransactions();
    await deposit(BigInt(1000)); // Deposit 1000 tokens
    await withdraw(BigInt(500)); // Withdraw 500 tokens
  } catch (error) {
    console.error('Error during transactions:', error);
  }

  balance = await connection.getBalance(payer.publicKey);
  console.log(`Final balance: ${balance / LAMPORTS_PER_SOL} SOL`);

  rl.close();
};

main().catch(console.error);
