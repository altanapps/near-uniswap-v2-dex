use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId};
use near_sdk::collections::{LookupMap, Vector};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    feeToSetter: AccountId,
    getPair: LookupMap<AccountId, LookupMap<AccountId, AccountId>>,
    allPairs: Vector<AccountId>,
}

impl Default for Contract{
    fn default() -> Self{
        Self{feeToSetter: "altan.testnet".parse().unwrap(),
        getPair: LookupMap::new(b"m"),
        allPairs: Vector::new(b"m")}
    }
} 

#[near_bindgen]
impl Contract { 
    #[init]
    #[private]
    pub fn init (_feeToSetter: AccountId) -> Self{
        Self{feeToSetter: _feeToSetter,
        getPair: LookupMap::new(b"m"),
        allPairs: Vector::new(b"m")}
    }

    #[payable]
    pub fn create_pair_and_deploy(tokenA: AccountId, tokenB: AccountId) -> AccountId{
        "altan.testnet".parse().unwrap()
    }

    #[private]
    pub fn create_pair_and_deploy_callback() -> bool {
        true
    }
}

