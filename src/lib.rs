use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
};
use borsh::{BorshDeserialize, BorshSerialize};

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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Ledger {
    pub participants: Vec<Participant>,
    pub productions: Vec<EnergyProduction>,
    pub demands: Vec<EnergyDemand>,
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
    let account_info_iter = &mut accounts.iter();
    let ledger_account = next_account_info(account_info_iter)?;

    if ledger_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut ledger = Ledger::try_from_slice(&ledger_account.data.borrow())?;

    match instruction {
        EnergyMarketInstruction::InitializeLedger => {
            ledger = Ledger {
                participants: Vec::new(),
                productions: Vec::new(),
                demands: Vec::new(),
            };
        }
        EnergyMarketInstruction::RegisterParticipant { participant_type } => {
            let participant_account = next_account_info(account_info_iter)?;
            ledger.participants.push(Participant {
                id: *participant_account.key,
                participant_type,
                wallet_balance: 0,
            });
        }
        EnergyMarketInstruction::ReportProduction { energy_amount, price } => {
            let producer_account = next_account_info(account_info_iter)?;
            ledger.productions.push(EnergyProduction {
                producer_id: *producer_account.key,
                energy_amount,
                price,
            });
        }
        EnergyMarketInstruction::PostDemand { energy_amount, price_limit } => {
            let consumer_account = next_account_info(account_info_iter)?;
            ledger.demands.push(EnergyDemand {
                consumer_id: *consumer_account.key,
                energy_amount,
                price_limit,
            });
        }
        EnergyMarketInstruction::MatchTransactions => {
            match_transactions(&mut ledger)?;
        }
        EnergyMarketInstruction::Deposit { amount } => {
            let participant_account = next_account_info(account_info_iter)?;
            if let Some(participant) = ledger.participants.iter_mut().find(|p| p.id == *participant_account.key) {
                participant.wallet_balance = participant.wallet_balance.checked_add(amount)
                    .ok_or(ProgramError::ArithmeticOverflow)?;
            } else {
                return Err(ProgramError::InvalidAccountData);
            }
        }
        EnergyMarketInstruction::Withdraw { amount } => {
            let participant_account = next_account_info(account_info_iter)?;
            if let Some(participant) = ledger.participants.iter_mut().find(|p| p.id == *participant_account.key) {
                if participant.wallet_balance < amount {
                    return Err(ProgramError::InsufficientFunds);
                }
                participant.wallet_balance = participant.wallet_balance.checked_sub(amount)
                    .ok_or(ProgramError::ArithmeticOverflow)?;
            } else {
                return Err(ProgramError::InvalidAccountData);
            }
        }
    }

    ledger_account.data.borrow_mut().copy_from_slice(&ledger.try_to_vec()?);
    Ok(())
}

fn match_transactions(ledger: &mut Ledger) -> ProgramResult {
    ledger.demands.sort_by(|a, b| b.energy_amount.cmp(&a.energy_amount));
    ledger.productions.sort_by(|a, b| a.price.cmp(&b.price));

    for demand in &mut ledger.demands {
        for production in &mut ledger.productions {
            if demand.energy_amount == 0 {
                break;
            }
            if demand.energy_amount <= production.energy_amount && demand.price_limit >= production.price {
                let trade_amount = demand.energy_amount.min(production.energy_amount);
                let total_cost = trade_amount.checked_mul(production.price)
                    .ok_or(ProgramError::ArithmeticOverflow)?;

                let consumer = ledger.participants.iter_mut()
                    .find(|p| p.id == demand.consumer_id)
                    .ok_or(ProgramError::InvalidAccountData)?;
                let producer = ledger.participants.iter_mut()
                    .find(|p| p.id == production.producer_id)
                    .ok_or(ProgramError::InvalidAccountData)?;

                if consumer.wallet_balance >= total_cost {
                    consumer.wallet_balance = consumer.wallet_balance.checked_sub(total_cost)
                        .ok_or(ProgramError::ArithmeticOverflow)?;
                    producer.wallet_balance = producer.wallet_balance.checked_add(total_cost)
                        .ok_or(ProgramError::ArithmeticOverflow)?;

                    demand.energy_amount = demand.energy_amount.checked_sub(trade_amount)
                        .ok_or(ProgramError::ArithmeticOverflow)?;
                    production.energy_amount = production.energy_amount.checked_sub(trade_amount)
                        .ok_or(ProgramError::ArithmeticOverflow)?;
                } else {
                    msg!("Insufficient balance for demand from {:?}", demand.consumer_id);
                }
            }
        }
    }

    ledger.productions.retain(|p| p.energy_amount > 0);
    ledger.demands.retain(|d| d.energy_amount > 0);

    Ok(())
}
