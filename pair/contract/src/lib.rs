// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{assert_one_yocto, Gas, PromiseError,
    ext_contract, env, log, near_bindgen, AccountId, Promise};
use near_sdk::json_types::U128;

pub mod external;
pub use crate::external::*;


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
        Self{
        reserve0: 0,
        reserve1: 0,
        token0:"altan.testnet".parse().unwrap(),
        token1:"dev-1675887648102-55313448745631".parse().unwrap(),
        factory: env::predecessor_account_id(),
        }
    }
}

#[near_bindgen]
impl Contract{

    #[init]
    #[private]
    pub fn init (_token0: AccountId, _token1: AccountId) -> Self {
        Self{
            reserve0: 0,
            reserve1: 0, 
            token0: _token0,
            token1: _token1,
            factory: env::predecessor_account_id()
        }
    }

    pub fn getBalance(&self, token: AccountId, account: AccountId) -> Promise {
        // Get the promise to read the contract balance
        let promise = ext_ft_contract::ext(token.clone())
        .with_static_gas(Gas(5*TGAS))
        .ft_balance_of(account);

        // Create a promise to callback getBalanceCallback
        return promise.then(
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(5*TGAS))
            .getBalanceCallback()
        )
    }

    pub fn getBalanceCallback(&self,#[callback_result] call_result: Result<U128, PromiseError>) -> U128 {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            log!("There was an error contacting the contract");
            return near_sdk::json_types::U128(0);  // what to do if you can't read it
        }
  
        // Return the greeting
        let balance: U128 = call_result.unwrap();
        balance
    }

    pub fn safeTransfer(&self, token: AccountId, to: AccountId, value: U128) {
        // Assert that the user has attached exactly 1 yoctoNEAR (for security reasons)
        assert_one_yocto();

        // Capture the sender as it is used here for a couple of times
        let sender: AccountId = env::predecessor_account_id();

        // Return the balance of the account
        let promise = self.getBalance(token, sender);

        // Assert that the user has the enough funds
        // assert!(balance > value, "There is not enough FT for the user to send money to the account");

        // Transfer the tokens from the sender to the user
        // ext_ft_contract::ext(token.clone())
        // .with_attached_deposit(1)
        // .ft_transfer(
        //     token, 
        //     value, 
        //     Some("Transfer FT into the Uniswap pair ".to_string()), //memo (to include some context)
        //  );
    }
}

