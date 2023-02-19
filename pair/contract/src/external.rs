use crate::*;

pub const TGAS: u64 = 1_000_000_000;

// FT interface, for cross-contract calls
#[ext_contract(ext_ft_contract)]
trait ExtFtContract {
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId, 
        amount: U128, 
        memo: Option<String>
    );

    fn ft_balance_of(&self, account_id: AccountId) -> U128;
}
