use crate::{Contract, ContractExt};
use near_sdk::{env, near};

#[near]
impl Contract {
    #[private]
    pub fn update_stored_contract(&mut self) {
        self.code = env::input();
    }

    pub fn get_code(&self) -> &Vec<u8> {
        self.code.as_ref().unwrap()
    }
}
