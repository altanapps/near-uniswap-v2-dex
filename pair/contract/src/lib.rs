// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{assert_one_yocto, Gas, PromiseError,
    ext_contract, env, log, near_bindgen, AccountId, Promise, PromiseOrValue};
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

    pub fn check_balance(&self, token: AccountId, account: AccountId, value: U128) -> Promise {
        // Get the promise to read the contract balance
        let promise = ext_ft_contract::ext(token)
        .with_static_gas(Gas(TGAS))
        .ft_balance_of(account);

        // Create a promise to callback getBalanceCallback
        return promise.then(
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(TGAS))
            .check_balance_callback(value)
        );
    }

    #[private]
    pub fn check_balance_callback(&self,
        #[callback_result] call_result: Result<U128, PromiseError>,
        value: U128) {
        // Check if the promise succeeded by calling the method outlined in external.rs
        if call_result.is_err() {
            log!("There was an error contacting the contract");
            // TODO: This needs to panic!
        }
  
        // Return the balance
        let balance: U128 = call_result.unwrap();

        // Assert that the value returned is greater than the value 
        assert!(balance >= value, "Balance needs to be greater than the value before transfer");

        // Log the balance of the contract just in case
        log!("The balance of this contract is {:?}", balance);
    }  

    #[private]
    pub fn transfer_tokens(&self, token: AccountId, receiver: AccountId, amount: U128) -> Promise {
        // This function transfers the tokens to this contract
        return ext_ft_contract::ext(token)
        .with_attached_deposit(1)
        .with_static_gas(Gas(TGAS))
        .ft_transfer(receiver, amount, Some("Transferring tokens \\ 
        to the AMM ".to_string()));
    }

    pub fn safe_transfer(&self, token: AccountId, value: U128) {
        // Assert that the user has attached exactly 1 yoctoNEAR (for security reasons)
        assert_one_yocto();

        // Capture the sender as it is used here for a couple of times
        let sender: AccountId = env::predecessor_account_id();

        // Return the balance of the account
        let promise = self.check_balance(token.clone(), sender, value);

        promise.then(
            Self::ext(env::current_account_id())
            .with_static_gas(Gas(TGAS))
            .transfer_tokens(token.clone(), env::current_account_id(), value)
        );
    }
}

