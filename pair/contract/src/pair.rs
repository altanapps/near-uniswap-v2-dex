use std::cmp::min;

use crate::utils::U256;
use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}, 
AccountId, Balance, collections::LookupMap, env};

// Define a variable to keep the number of tokens in the pair
const NUM_TOKENS: usize = 2;

pub struct Pair {
    // List of tokens in the pair
    pub token_account_ids: Vec<AccountId>,

    // How much NEAR does this contract have
    pub amounts: Vec<Balance>,

    // Volumes accumulated by this pair
    pub volumes: Vec<Balance>,

    // Fee charged per swap
    pub fee: u32,

    // Shares of the pool by the liquidity providers
    pub shares: LookupMap<AccountId, Balance>,

    // Total number of shares
    pub total_shares: Balance,
}

impl Pair {
    // Initialize the pair with the given id, tokens, and fee
    pub fn new(id: u32, token_account_ids: Vec<AccountId>, fee: u32) -> Self {
        assert_eq!(token_account_ids.len(), NUM_TOKENS, "Pair must have exactly 2 tokens");
        Self {
            token_account_ids,
            amounts: vec![0; NUM_TOKENS],
            volumes: vec![0; NUM_TOKENS],
            fee,
            // TODO: Find the StorageKey in the original code
            shares: LookupMap::new(StorageKey::Shares {
                pool_id: id,
            }),
            total_shares: 0,
        }
    }

    // See if an account has been registered as a liquidity provider
    pub fn is_liquidity_provider(&self, account_id: &AccountId) -> bool {
        self.shares.contains_key(account_id)
    }

    // Register the LP provider
    pub fn register_liquidity_provider(&mut self, account_id: &AccountId) {
        assert!(!self.is_liquidity_provider(account_id), "Account is already a liquidity provider");
        self.shares.insert(account_id, &0);
    }

    // Transfer shares from the precedessor to receiver
    pub fn transfer_shares(&mut self, predecessor_id: &AccountId, receiver_id: &AccountId, amount: u128) {
        assert!(self.is_liquidity_provider(predecessor_id), "Account is not a liquidity provider");
        assert!(self.is_liquidity_provider(receiver_id), "Receiver is not a liquidity provider");

        // Register the LP provider
        let balance = self.shares.get(predecessor_id).unwrap();

        // Check that the sender has enough balance
        assert!(balance >= amount, "Not enough balance");

        // Calculate the new balance for the predecessor
        let new_balance = balance - amount;

        // Update the balance for the predecessor
        self.shares.insert(predecessor_id, &new_balance);

        // Get the balance for the receiver
        let balance = self.shares.get(receiver_id).unwrap();

        // Calculate the new balance for the receiver
        let new_balance = balance + amount;

        // Update the balance for the receiver
        self.shares.insert(receiver_id, &new_balance);
    }


    // Return the balance for an account
    pub fn get_balance(&self, account_id: &AccountId) -> Balance {
        self.shares.get(account_id).unwrap_or(0)
    }

    // Return the total number of shares
    pub fn get_total_shares(&self) -> Balance {
        self.total_shares
    }

    // Return the list of tokens in the pool
    pub fn get_tokens(&self) -> Vec<AccountId> {
        self.token_account_ids.clone()
    }

    // WARNING: This function is not the same as the Ref.finance code
    // Adds the amount of tokens to the pair, returns the number of shares that were minted, and
    // Updates amount to amount kept in the pool
    pub fn add_liquidity(&mut self, sender_id: &AccountId, amounts: Vec<Balance>) -> Balance {
        // Ensures that the number provided is correct
        assert_eq!(amounts.len(), NUM_TOKENS, "Pair must have exactly 2 tokens");

        // Calculate the number of shares to mint
        let shares = if self.total_shares == 0 {
            // If there are no shares, mint the number of shares equal to the smallest amount
            min(amounts[0], amounts[1])
        } else {
            // Otherwise, mint the number of shares equal to the smallest amount times the total
            // number of shares divided by the smallest amount in the pool
            min(amounts[0] * self.total_shares / self.amounts[0], amounts[1] * self.total_shares / self.amounts[1])
        };

        // Update the total number of shares
        self.total_shares += shares;

        // Update the amount of tokens in the pool
        for i in 0..NUM_TOKENS {
            self.amounts[i] += amounts[i];
        }

        // Return the number of shares minted
        shares
    }   

    // Mint shares for given user
    pub fn mint_shares(&mut self, account_id: &AccountId, shares: Balance) {
        // Get the balance for the account
        let balance = self.shares.get(account_id).unwrap();

        // Calculate the new balance for the account
        let new_balance = balance + shares;

        // Update the balance for the account
        self.shares.insert(account_id, &new_balance);
    }

    // Removes amount of tokens from the pair given number of shares, and returns amounts to the parent
    pub fn remove_liquidity(&mut self, account_id: &AccountId, shares: Balance) -> Vec<Balance> {
        // Calculate the amount of tokens to return
        let amounts: Vec<Balance> = self.amounts.iter().map(|amount| amount * shares / self.total_shares).collect();

        // Update the total number of shares
        self.total_shares -= shares;

        // Update the amount of tokens in the pool
        for i in 0..NUM_TOKENS {
            self.amounts[i] -= amounts[i];
        }

        // Return the amount of tokens to return
        amounts
    }

    // Find the token index for a given token
    pub fn find_token_index(&self, token_account_id: &AccountId) -> Option<usize> {
        // TODO: What happens if you don't have the token?
        self.token_account_ids.iter().position(|id| id == token_account_id)
    }

    // Write the code depending on the ref.finance code
    fn internal_get_return(&self, token_in: usize, token_out: usize, amount: Balance) -> Balance {
        let in_balance = U256::from(self.amounts[token_in]);
        let out_balance = U256::from(self.amounts[token_out]);
        assert!(in_balance > U256::zero() && out_balance > U256::zero(), "Pool is empty");
        assert!(amount > 0, "Amount must be positive");
        assert!(token_in != token_out, "Tokens must be different");

        // Find the amount with fee
        // TODO: Correct the code looking at ref.finance code
        let amount_with_fee = U256::from(amount) * (10000 - self.fee);
        amount_with_fee.as_u128()
    }

    // Return the amount of tokens to return
    pub fn get_return(&self, token_in: &AccountId, token_out: &AccountId, amount: Balance) -> Balance {
        let token_in_index = self.find_token_index(token_in).unwrap();
        let token_out_index = self.find_token_index(token_out).unwrap();
        self.internal_get_return(token_in_index, token_out_index, amount)
    }

    // Find the pool's total fee
    pub fn get_fee(&self) -> u32 {
        self.fee
    }

    // Return the amount of tokens in the pool
    pub fn get_amounts(&self) -> Vec<Balance> {
        self.amounts.clone()
    }

    // Swap the token_amount_in of token_in into token_out, and return how much was received
    pub fn swap(&mut self, token_in: &AccountId, 
        token_amount_in: Balance, token_out: &AccountId,
        min_amount_out: Balance) -> Balance {
        assert_ne!(token_in, token_out, "Tokens must be different");
        assert!(token_amount_in > 0, "Amount must be positive");

        // Find the token indexes
        let token_in_index = self.find_token_index(token_in).unwrap();
        let token_out_index: usize = self.find_token_index(token_out).unwrap();
        let amount_out = self.internal_get_return(token_in_index, token_out_index, token_amount_in);
        assert!(amount_out >= min_amount_out, "Not enough amount out");

        // Log the amount of token_ins swapped for token_outs
        env::log(format!("Swapped {} {} for {} {}", token_amount_in, token_in, amount_out, token_out).as_bytes());

        // Update the amount of tokens in the pool
        self.amounts[token_in_index] += token_amount_in;
        self.amounts[token_out_index] -= amount_out;

        // Return the amount of tokens received
        amount_out
    }

}



