// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId, Promise};

const MINIMUM_LIQUIDITY: i32 = 1000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    reserve0: i32, 
    reserve1: i32,
    token0: AccountId,
    token1: AccountId, 
    factory: AccountId,
}

impl Default for Contract{
    fn default() -> Self{
        reserve0: 0,
        reserve1: 0,
        token0:"altan.testnet".parse().unwrap(),
        token1:"dev-1675887648102-55313448745631".parse().unwrap()
    }
}

#[near_bindgen]
impl Contract{
    #[init]
    #[private]
    pub fn init (_token0: AccountId, _token1: AccountId) -> Self{
        Self{
            reserve0: 0,
            reserve1: 0, 
            token0: _token0,
            token1: _token1,
        }
    }

    pub fn _safeTransfer(token: AccountId, to: AccountId, i32 value) {
        // Call the token contract
        // A
    }


}

