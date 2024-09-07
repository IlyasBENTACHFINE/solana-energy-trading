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

// Define the program ID
solana_program::declare_id!("EnergyTradingProgramID11111111111111111111111111");

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
    pub energy_balance: i64,
    pub token_balance: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct EnergyOffer {
    pub producer_id: Pubkey,
    pub energy_amount: u64,
    pub price: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct EnergyDemand {
    pub consumer_id: Pubkey,
    pub energy_amount: u64,
    pub max_price: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MarketState {
    pub participants: Vec<Participant>,
    pub energy_offers: Vec<EnergyOffer>,
    pub energy_demands: Vec<EnergyDemand>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum EnergyMarketInstruction {
    InitializeMarket,
    RegisterParticipant { participant_type: ParticipantType },
    SubmitEnergyOffer { energy_amount: u64, price: u64 },
    SubmitEnergyDemand { energy_amount: u64, max_price: u64 },
    MatchOffers,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = EnergyMarketInstruction::try_from_slice(instruction_data)?;

    match instruction {
        EnergyMarketInstruction::InitializeMarket => initialize_market(accounts, program_id),
        EnergyMarketInstruction::RegisterParticipant { participant_type } => {
            register_participant(accounts, program_id, participant_type)
        }
        EnergyMarketInstruction::SubmitEnergyOffer { energy_amount, price } => {
            submit_energy_offer(accounts, program_id, energy_amount, price)
        }
        EnergyMarketInstruction::SubmitEnergyDemand { energy_amount, max_price } => {
            submit_energy_demand(accounts, program_id, energy_amount, max_price)
        }
        EnergyMarketInstruction::MatchOffers => match_offers(accounts, program_id),
    }
}

fn initialize_market(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let market_account = next_account_info(account_info_iter)?;

    if market_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let market_state = MarketState {
        participants: Vec::new(),
        energy_offers: Vec::new(),
        energy_demands: Vec::new(),
    };

    market_state.serialize(&mut &mut market_account.data.borrow_mut()[..])?;

    msg!("Energy market initialized");
    Ok(())
}

fn register_participant(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    participant_type: ParticipantType,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let market_account = next_account_info(account_info_iter)?;
    let participant_account = next_account_info(account_info_iter)?;

    if market_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut market_state = MarketState::try_from_slice(&market_account.data.borrow())?;

    let new_participant = Participant {
        id: *participant_account.key,
        participant_type,
        energy_balance: 0,
        token_balance: 1000, // Initial balance for simplicity
    };

    market_state.participants.push(new_participant);

    market_state.serialize(&mut &mut market_account.data.borrow_mut()[..])?;

    msg!("Participant registered successfully");
    Ok(())
}

fn submit_energy_offer(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    energy_amount: u64,
    price: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let market_account = next_account_info(account_info_iter)?;
    let producer_account = next_account_info(account_info_iter)?;

    if market_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut market_state = MarketState::try_from_slice(&market_account.data.borrow())?;

    let offer = EnergyOffer {
        producer_id: *producer_account.key,
        energy_amount,
        price,
    };

    market_state.energy_offers.push(offer);

    market_state.serialize(&mut &mut market_account.data.borrow_mut()[..])?;

    msg!("Energy offer submitted successfully");
    Ok(())
}

fn submit_energy_demand(
    accounts: &[AccountInfo],
    program_id: &Pubkey,
    energy_amount: u64,
    max_price: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let market_account = next_account_info(account_info_iter)?;
    let consumer_account = next_account_info(account_info_iter)?;

    if market_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut market_state = MarketState::try_from_slice(&market_account.data.borrow())?;

    let demand = EnergyDemand {
        consumer_id: *consumer_account.key,
        energy_amount,
        max_price,
    };

    market_state.energy_demands.push(demand);

    market_state.serialize(&mut &mut market_account.data.borrow_mut()[..])?;

    msg!("Energy demand submitted successfully");
    Ok(())
}

fn match_offers(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let market_account = next_account_info(account_info_iter)?;

    if market_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut market_state = MarketState::try_from_slice(&market_account.data.borrow())?;

    // Sort offers by price (ascending) and demands by price (descending)
    market_state.energy_offers.sort_by(|a, b| a.price.cmp(&b.price));
    market_state.energy_demands.sort_by(|a, b| b.max_price.cmp(&a.max_price));

    let mut matched_trades = Vec::new();

    for demand in &market_state.energy_demands {
        for offer in &market_state.energy_offers {
            if offer.price <= demand.max_price && offer.energy_amount > 0 {
                let trade_amount = std::cmp::min(demand.energy_amount, offer.energy_amount);
                let trade_price = offer.price;

                // Update participant balances
                if let Some(consumer) = market_state.participants.iter_mut().find(|p| p.id == demand.consumer_id) {
                    consumer.energy_balance += trade_amount as i64;
                    consumer.token_balance -= trade_amount * trade_price;
                }
                if let Some(producer) = market_state.participants.iter_mut().find(|p| p.id == offer.producer_id) {
                    producer.energy_balance -= trade_amount as i64;
                    producer.token_balance += trade_amount * trade_price;
                }

                matched_trades.push((offer.producer_id, demand.consumer_id, trade_amount, trade_price));

                // Update remaining energy in the offer
                if let Some(offer_to_update) = market_state.energy_offers.iter_mut().find(|o| o.producer_id == offer.producer_id) {
                    offer_to_update.energy_amount -= trade_amount;
                }
            }
        }
    }

    // Remove completed offers and demands
    market_state.energy_offers.retain(|o| o.energy_amount > 0);
    market_state.energy_demands.retain(|d| d.energy_amount > 0);

    market_state.serialize(&mut &mut market_account.data.borrow_mut()[..])?;

    msg!("Offers matched successfully");
    Ok(())
}
