use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::{env, near_bindgen, ext_contract, Gas, Promise, log};
use serde::Deserialize;
//use serde::{Serialize, Deserialize};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Daobot {
    records: LookupMap<String, String>,
}


#[derive(Deserialize)]
#[derive(Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    id: u64,
}


// Trigger macro to create interfact to external contract.
#[ext_contract(ext_astrodao)]
pub trait Astrodao {
     fn version(&self) -> String;
     fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<Proposal>;
    fn get_proposal(&self, id: U64) -> Proposal;
     fn act_proposal(&self, id: U64, action: String);
}



// Recieve callbacks from external contract.
#[ext_contract(ext_self)]
trait Callbacks {
    fn on_get_proposals(&self, dao_id: AccountId, #[callback] proposals: Vec<Proposal>);
    fn on_get_proposal(&self,#[callback] proposal: Proposal);
    fn panic_debug(&self);
}   

impl Default for Daobot {
    fn default() -> Self {
        Self {
            records: LookupMap::new(b"r".to_vec()),
        }
    }
}


// Gas needed for common operations.
pub const GAS_FOR_COMMON_OPERATIONS: Gas = 30_000_000_000_000;

// Gas reserved for the current call.
pub const GAS_RESERVED_FOR_CURRENT_CALL: Gas = 20_000_000_000_000;

pub const GAS_ESTIMATE: Gas = 10_000_000_000;
#[near_bindgen]
impl Daobot {

    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(&account_id, &message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        return self.records.get(&account_id);
    }

    pub fn something(&self, arg1: String) -> String {
        return "Something".to_string() + &arg1;
    }

    pub fn panic_debug(&self) {
        log!("Gas {:?}", env::prepaid_gas() - env::used_gas());
    }

    pub fn approve_members(&self, dao_id: String) {
        let total_gas = env::prepaid_gas();
        let num_calls = 6;
        let gas_per_call = total_gas / num_calls;

        log!("Used gas: {:?} out of {:?}", env::used_gas(), env::prepaid_gas());
        let callback = ext_self::on_get_proposals(dao_id.clone(),&env::current_account_id(), 0, 28860827153770);
        log!("Callback defined,  gas: {:?} out of {:?}", env::used_gas(), env::prepaid_gas());
        ext_astrodao::get_proposals(0, 1, &dao_id, 0, gas_per_call )
        .then(callback);
        log!("After get_proposals Used gas: {:?} out of {:?}", env::used_gas(), env::prepaid_gas());

            
        // .then(
        // 
        }

    // #[private]
    pub fn on_get_proposals(&self, dao_id: &near_sdk::AccountId, #[callback] proposals: Vec<Proposal>)  {
        // panic!("Gas Available {:?}",  env::prepaid_gas() - env::used_gas());
        // let last_proposal_id = proposals.last().map(|p| p.id).unwrap_or(0);
        // ext_astrodao::act_proposal(U64(last_proposal_id), "VoteApprove".to_string(), &dao_id, 0, 288608271537707-GAS_FOR_COMMON_OPERATIONS);
      1+1;
    }
}


#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Daobot::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Daobot::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}
