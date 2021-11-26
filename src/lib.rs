use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, Promise};
use near_sdk::collections::{UnorderedMap};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VotingContract {
    // SETUP CONTRACT STATE
    candidates : UnorderedMap<String, i128>,
    voters : UnorderedMap<String, String>
}

impl Default for VotingContract {
    fn default() -> Self {
        Self {
            candidates: UnorderedMap::new(b"a"),
            voters: UnorderedMap::new(b"b")
        }
    }
}


#[near_bindgen]
impl VotingContract {
    // ADD CONTRACT METHODS HERE

    // #[init]
    // pub fn new_default_meta() -> Self {
    //     Self::new()
    // }

    // /// Initializes the contract with the given total supply owned by the given `owner_id` with
    // /// the given fungible token metadata.
    // #[init]
    // pub fn new() -> Self {
    //     assert!(!env::state_exists(), "Already initialized");
        
    //     let this = Self {
    //         candidates: UnorderedMap::new(b"a"),
    //         voters: UnorderedMap::new(b"b"),
    //     };
    //     this
    // }

    pub fn help(&mut self) -> Vec<String> {
        return [
            "Add  Candidate : near call cha7.bedeawi.testnet add_candidate '{\"name\": \"Wagih\"}' --accountId $ID".to_string(),
            "List Candidates: near call cha7.bedeawi.testnet get_candidate '' --accountId $ID".to_string(),
            "Add  Vote      : near call cha7.bedeawi.testnet add_vote '{\"name\": \"Wagih\"}' --accountId $ID".to_string(),
            "Get  Stats     : near call cha7.bedeawi.testnet get_stats '' --accountId $ID".to_string(),
        ].to_vec();
    }

    pub fn add_candidate(&mut self, name: String) -> String {
        
        if !self.candidates.get(&name).is_none() {
           return "Candidate already exists".to_string(); 
        }

        self.candidates.insert(&name, &0);
        return ["Added", &name,"!"].join(" ");
    }

    pub fn get_candidate(&mut self) -> Vec<String> {
        
        return self.candidates.keys_as_vector().to_vec().clone();
        
    }

    pub fn get_stats(&mut self) -> Vec<(String, i128)> {
        
        return self.candidates.iter().map(|x|x).collect();
        
    }

    pub fn add_vote(&mut self, name: String) -> String {
        
        if self.candidates.get(&name).is_none() {
            return "Candidate doesn't exist".to_string();
        }

        if !self.voters.get(&env::signer_account_id()).is_none() {
            return "You have already voted".to_string();
        }

        self.voters.insert(&env::signer_account_id() ,&name);

        let mut votes_count = self.candidates.get(&name).unwrap();
        votes_count += 1;
        self.candidates.insert(&name, &votes_count);
        return ["Voted For", &name, "Total of", &votes_count.to_string() ,"!"].join(" ");
    }
}


// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs};
    use near_sdk::{testing_env, VMContext, AccountId};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
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
            epoch_height: 19,
        }
    }

    // TESTS HERE

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn helloworld() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = VotingContract {};
        let msg = contract.add_candidate("WAGIH".to_string());
        println!("Value after calling is: {}", msg);
        // confirm that we received 1 when calling get_num
        assert_eq!(1, 1);
    }
}
