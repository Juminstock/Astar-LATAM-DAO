#![cfg_attr(not(feature = "std"), no_std)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]

#[ink::contract]
mod dao {
    use ink::storage::Mapping;

    #[ink(event)]
    pub struct Vote {
        #[ink(topic)]
        candidate: AccountId,
    }

    #[ink(event)]
    pub struct NewCandidate {
        #[ink(topic)]
        candidate: AccountId,
    }
    
    #[derive(Debug)]
    #[ink::storage_item]
    pub struct Admin {
        address: AccountId,
        modified_date: u64
    }

    #[ink(storage)]
    pub struct Dao {
        admin: Admin,
        votes: Mapping<AccountId, u64>,
        enabled_candidates: Mapping<AccountId, ()>
    }
    impl Dao {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { votante: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        #[ink(message)]
        pub fn get_votante(&mut self) -> bool {
            self.votante
        }
    }
}
