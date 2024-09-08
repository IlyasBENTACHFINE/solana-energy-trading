use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    clock::Clock,
    sysvar::Sysvar,
};
use borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

// Define the program ID
solana_program::declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ParticipantType {
    Producer,
    Consumer,
    Prosumer,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Participant {
    pub id: Pubkey,
    pub participant_type: ParticipantType,
    pub wallet_balance: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct EnergyProduction {
    pub producer_id: Pubkey,
    pub energy_amount: u64,
    pub price: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct EnergyDemand {
    pub consumer_id: Pubkey,
    pub energy_amount: u64,
    pub price_limit: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Transaction {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub price: u64,
    pub timestamp: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Ledger {
    pub participants: Vec<Participant>,
    pub productions: Vec<EnergyProduction>,
    pub demands: Vec<EnergyDemand>,
    pub transactions: Vec<Transaction>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum EnergyMarketInstruction {
    InitializeLedger,
    RegisterParticipant { participant_type: ParticipantType },
    ReportProduction { energy_amount: u64, price: u64 },
    PostDemand { energy_amount: u64, price_limit: u64 },
    MatchTransactions,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = EnergyMarketInstruction::try_from_slice(instruction_data)?;

    match instruction {
        EnergyMarketInstruction::InitializeLedger => initialize_ledger(program_id, accounts),
        EnergyMarketInstruction::RegisterParticipant { participant_type } => {
            register_participant(program_id, accounts, participant_type)
        }
        EnergyMarketInstruction::ReportProduction { energy_amount, price } => {
            report_energy_production(program_id, accounts, energy_amount, price)
        }
        EnergyMarketInstruction::PostDemand { energy_amount, price_limit } => {
            post_energy_demand(program_id, accounts, energy_amount, price_limit)
        }
        EnergyMarketInstruction::MatchTransactions => match_transactions(program_id, accounts),
        EnergyMarketInstruction::Deposit { amount } => deposit(program_id, accounts, amount),
        EnergyMarketInstruction::Withdraw { amount } => withdraw(program_id, accounts, amount),
    }
}

fn initialize_ledger(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let ledger_account = next_account_info(account_info_iter)?;

    if ledger_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let ledger = Ledger {
        participants: Vec::new(),
        productions: Vec::new(),
        demands: Vec::new(),
        transactions: Vec::new(),
    };

    ledger.serialize(&mut &mut ledger_account.data.borrow_mut()[..])?;

    Ok(())
}

fn register_participant(program_id: &Pubkey, accounts: &[AccountInfo], participant_type: ParticipantType) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let participant_account = next_account_info(account_info_iter)?;
    let ledger_account = next_account_info(account_info_iter)?;

    if ledger_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut ledger = Ledger::try_from_slice(&ledger_account.data.borrow())?;

    let new_participant = Participant {
        id: *participant_account.key,
        participant_type,
        wallet_balance: 0,
    };

    ledger.participants.push(new_participant);

    ledger.serialize(&mut &mut ledger_account.data.borrow_mut()[..])?;

    Ok(())
}

fn report_energy_production(program_id: &Pubkey, accounts: &[AccountInfo], energy_amount: u64, price: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let producer_account = next_account_info(account_info_iter)?;
    let ledger_account = next_account_info(account_info_iter)?;

    if ledger_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut ledger = Ledger::try_from_slice(&ledger_account.data.borrow())?;

    if !ledger.participants.iter().any(|p| p.id == *producer_account.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let production = EnergyProduction {
        producer_id: *producer_account.key,
        energy_amount,
        price,
    };

    ledger.productions.push(production);

    ledger.serialize(&mut &mut ledger_account.data.borrow_mut()[..])?;

    Ok(())
}

fn post_energy_demand(program_id: &Pubkey, accounts: &[AccountInfo], energy_amount: u64, price_limit: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let consumer_account = next_account_info(account_info_iter)?;
    let ledger_account = next_account_info(account_info_iter)?;

    if ledger_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut ledger = Ledger::try_from_slice(&ledger_account.data.borrow())?;

    if !ledger.participants.iter().any(|p| p.id == *consumer_account.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    let demand = EnergyDemand {
        consumer_id: *consumer_account.key,
        energy_amount,
        price_limit,
    };

    ledger.demands.push(demand);

    ledger.serialize(&mut &mut ledger_account.data.borrow_mut()[..])?;

    Ok(())
}

fn match_transactions(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let ledger_account = next_account_info(account_info_iter)?;

    if ledger_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut ledger = Ledger::try_from_slice(&ledger_account.data.borrow())?;

    ledger.demands.sort_by(|a, b| b.energy_amount.cmp(&a.energy_amount));
    ledger.productions.sort_by(|a, b| a.price.cmp(&b.price));

    let mut matched_trades = Vec::new();

    for demand in &mut ledger.demands {
        for production in &mut ledger.productions {
            if demand.energy_amount == 0 {
                break;
            }
            if demand.energy_amount <= production.energy_amount && demand.price_limit >= production.price {
                let trade_amount = demand.energy_amount.min(production.energy_amount);
                let trade_price = production.price;
                let total_cost = trade_amount.checked_mul(trade_price)
                    .ok_or(ProgramError::ArithmeticOverflow)?;

                // Store the IDs instead of references
                let consumer_id = demand.consumer_id;
                let producer_id = production.producer_id;

                // Perform the trade if the consumer has enough balance
                if let (Some(consumer), Some(producer)) = (
                    ledger.participants.iter_mut().find(|p| p.id == consumer_id),
                    ledger.participants.iter_mut().find(|p| p.id == producer_id)
                ) {
                    if consumer.wallet_balance >= total_cost {
                        consumer.wallet_balance = consumer.wallet_balance.checked_sub(total_cost)
                            .ok_or(ProgramError::ArithmeticOverflow)?;
                        producer.wallet_balance = producer.wallet_balance.checked_add(total_cost)
                            .ok_or(ProgramError::ArithmeticOverflow)?;

                        demand.energy_amount = demand.energy_amount.checked_sub(trade_amount)
                            .ok_or(ProgramError::ArithmeticOverflow)?;
                        production.energy_amount = production.energy_amount.checked_sub(trade_amount)
                            .ok_or(ProgramError::ArithmeticOverflow)?;

                        matched_trades.push(Transaction {
                            from: consumer_id,
                            to: producer_id,
                            amount: trade_amount,
                            price: trade_price,
                            timestamp: Clock::get()?.unix_timestamp,
                        });
                    } else {
                        msg!("Insufficient balance for demand from {:?}", consumer_id);
                    }
                }
            }
        }
    }

    ledger.productions.retain(|p| p.energy_amount > 0);
    ledger.demands.retain(|d| d.energy_amount > 0);
    ledger.transactions.extend(matched_trades);

    ledger.serialize(&mut &mut ledger_account.data.borrow_mut()[..])?;

    Ok(())
}

fn deposit(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let participant_account = next_account_info(account_info_iter)?;
    let ledger_account = next_account_info(account_info_iter)?;

    if ledger_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut ledger = Ledger::try_from_slice(&ledger_account.data.borrow())?;

    if let Some(participant) = ledger.participants.iter_mut().find(|p| p.id == *participant_account.key) {
        participant.wallet_balance = participant.wallet_balance.checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
    } else {
        return Err(ProgramError::InvalidAccountData);
    }

    ledger.serialize(&mut &mut ledger_account.data.borrow_mut()[..])?;

    Ok(())
}

fn withdraw(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let participant_account = next_account_info(account_info_iter)?;
    let ledger_account = next_account_info(account_info_iter)?;

    if ledger_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut ledger = Ledger::try_from_slice(&ledger_account.data.borrow())?;

    if let Some(participant) = ledger.participants.iter_mut().find(|p| p.id == *participant_account.key) {
        if participant.wallet_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }
        participant.wallet_balance = participant.wallet_balance.checked_sub(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
    } else {
        return Err(ProgramError::InvalidAccountData);
    }

    ledger.serialize(&mut &mut ledger_account.data.borrow_mut()[..])?;

    Ok(())
}
