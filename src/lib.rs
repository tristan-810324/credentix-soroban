#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, String};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Unauthorized = 1,
    InvalidScore = 2,
    CredentialAlreadyExists = 3,
    NotCertified = 4,
}

#[contracttype]
pub enum DataKey {
    Issuer,
    CredentialRecord(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialMetadata {
    pub score: u32,
    pub grade: Symbol,
    pub timestamp: u64,
    pub is_soulbound: bool, // SBT flag: non-transferable
}

#[contract]
pub struct CredentiXAdvancedContract;

#[contractimpl]
impl CredentiXAdvancedContract {
    /// Initialize the contract with an authorized issuer (admin)
    pub fn initialize(env: Env, issuer: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Issuer) {
            return Err(Error::Unauthorized);
        }
        env.storage().instance().set(&DataKey::Issuer, &issuer);
        Ok(())
    }

    /// Mint a Soulbound Credential Certificate on-chain
    pub fn mint_credential(
        env: Env,
        issuer: Address,
        student: Address,
        score: u32,
    ) -> Result<CredentialMetadata, Error> {
        issuer.require_auth();

        let stored_issuer: Address = env.storage().instance().get(&DataKey::Issuer).unwrap();
        if issuer != stored_issuer {
            return Err(Error::Unauthorized);
        }

        if score > 100 {
            return Err(Error::InvalidScore);
        }

        let key = DataKey::CredentialRecord(student.clone());
        if env.storage().instance().has(&key) {
            return Err(Error::CredentialAlreadyExists);
        }

        let grade = if score >= 90 {
            Symbol::new(&env, "A_EXCELLENT")
        } else if score >= 75 {
            Symbol::new(&env, "B_CREDIT")
        } else if score >= 60 {
            Symbol::new(&env, "C_PASS")
        } else {
            Symbol::new(&env, "F_FAIL")
        };

        let metadata = CredentialMetadata {
            score,
            grade,
            timestamp: env.ledger().timestamp(),
            is_soulbound: true, // Non-transferable token guarantee
        };

        env.storage().instance().set(&key, &metadata);

        // Real-time Event Publishing for Frontend Sync
        let topics = (Symbol::new(&env, "cred_minted"), student.clone());
        env.events().publish(topics, (score, metadata.is_soulbound));

        Ok(metadata)
    }

    /// Verify if a student holds a valid Soulbound Credential
    pub fn verify_credential(env: Env, student: Address) -> CredentialMetadata {
        env.storage()
            .instance()
            .get(&DataKey::CredentialRecord(student))
            .unwrap_or(CredentialMetadata {
                score: 0,
                grade: Symbol::new(&env, "NONE"),
                timestamp: 0,
                is_soulbound: false,
            })
    }
}