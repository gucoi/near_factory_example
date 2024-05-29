use near_sdk::{near, Gas, NearToken};

mod deploy;
mod manager;

const NEAR_PER_STORAGE: NearToken = NearToken::from_yoctonear(10u128.pow(18));
const DEFAULT_CONTRACT: &[u8] = include_bytes!("./donation_contract/donation.wasm");
const TGAS: Gas = Gas::from_tgas(1);
const NO_DEPOSIT: NearToken = NearToken::from_near(0);

#[near(contract_state)]
pub struct Contract {
    code: Option<Vec<u8>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            code: Some(DEFAULT_CONTRACT.to_vec()),
        }
    }
}
