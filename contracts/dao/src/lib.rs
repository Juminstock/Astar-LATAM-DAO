#![cfg_attr(not(feature = "std"), no_std)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]

#[ink::contract]
mod dao {
    use ink::storage::Mapping;

    //Evento de voto
    #[ink(event)]
    pub struct Vote {
        #[ink(topic)]
        candidate: AccountId
    }

    //Evento de nuevo candidato
    #[ink(event)]
    pub struct NewCandidate {
        #[ink(topic)]
        candidate: AccountId
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
        pub fn new(admin: AccountId) -> Self {
            let now = Self::env().block_timestamp();
            Self {
                admin: Admin {
                    address: admin,
                    modified_date: now,
                },
                votes: Mapping::default(),
                enabled_candidates: Mapping::default()
            }
        }

        #[ink(message)]
        pub fn add_candidate(&mut self, candidate: AccountId) {
            assert!(self.env().caller() == self.admin.address);

            self.enabled_candidates.insert(candidate, &());
            self.env().emit_event(NewCandidate { candidate });
        }

        #[ink(message)]
        pub fn vote(&mut self, candidate: AccountId) {
            assert!(self.enabled_candidates.contains(candidate));

            let candidate_votes = self.votes.get(candidate).unwrap_or(0);
            self.votes.insert(candidate, &(candidate_votes + 1));
            self.env().emit_event(Vote { candidate });
        }

        #[ink(message)]
        pub fn get_votes(&self, candidate: AccountId) {
            self.votes.get(candidate);

        }
    }
}
