use std::cmp::min;

use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}, AccountId, Balance, collections::LookupMap};

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
    pub fn new(token_account_ids: Vec<AccountId>, fee: u32) -> Self {
        assert_eq!(token_account_ids.len(), NUM_TOKENS, "Pair must have exactly 2 tokens");
        Self {
            token_account_ids,
            amounts: vec![0; NUM_TOKENS],
            volumes: vec![0::; NUM_TOKENS],
            fee,
            // [AUDIT_11]
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
        self.shares.insert(account_id, 0);
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

    

}



