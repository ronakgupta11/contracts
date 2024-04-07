#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod liquidity_loan_contract {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        collections::{
            HashMap as StorageHashMap,
            Vec as StorageVec,
        },
        lazy::Lazy,
    };

    #[ink(storage)]
    pub struct LiquidityLoanContract {
        // Mapping to store liquidity provided by each address
        liquidity_provided: StorageHashMap<AccountId, Balance>,
        // Mapping to store loan taken by each address
        loan_taken: StorageHashMap<AccountId, Balance>,
    }

    impl LiquidityLoanContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                liquidity_provided: StorageHashMap::new(),
                loan_taken: StorageHashMap::new(),
            }
        }

        // Function to provide liquidity
        #[ink(message)]
        pub fn provide_liquidity(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let balance = self.liquidity_provided.entry(caller).or_insert(0);
            *balance += amount;
        }

        // Function to take a loan
        #[ink(message)]
        pub fn take_loan(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let balance = self.loan_taken.entry(caller).or_insert(0);
            *balance += amount;
        }

        // Function to get liquidity provided by a specific address
        #[ink(message)]
        pub fn get_liquidity_provided(&self, address: AccountId) -> Option<Balance> {
            self.liquidity_provided.get(&address).copied()
        }

        // Function to get loan taken by a specific address
        #[ink(message)]
        pub fn get_loan_taken(&self, address: AccountId) -> Option<Balance> {
            self.loan_taken.get(&address).copied()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn provide_liquidity_works() {
            let mut contract = LiquidityLoanContract::new();
            contract.provide_liquidity(100);
            assert_eq!(contract.get_liquidity_provided(AccountId::from([0x1; 32])), Some(100));
        }

        #[ink::test]
        fn take_loan_works() {
            let mut contract = LiquidityLoanContract::new();
            contract.take_loan(50);
            assert_eq!(contract.get_loan_taken(AccountId::from([0x2; 32])), Some(50));
        }
    }
}
