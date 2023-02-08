use near_sdk::{ext_contract, AccountId};

pub const TGAS: u64 = 1_000_000_000;

// FT interface, for cross-contract calls
#[ext_contract(fungible_token)]
trait FungibleToken {
  fn ft_transfer(&self, receiverId: AccountId, amount: i32);
}