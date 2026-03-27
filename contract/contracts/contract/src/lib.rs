#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec, String};

#[derive(Clone)]
#[contracttype]
pub struct AuditRecord {
    pub auditor: String,
    pub timestamp: u64,
    pub report_hash: String,
}

#[contract]
pub struct AuditRegistry;

#[contractimpl]
impl AuditRegistry {

    // Store key for all audit records
    fn get_storage_key() -> Symbol {
        Symbol::new("AUDITS")
    }

    // Add a new audit record
    pub fn add_audit(env: Env, auditor: String, report_hash: String) {
        let mut audits: Vec<AuditRecord> = env
            .storage()
            .instance()
            .get(&Self::get_storage_key())
            .unwrap_or(Vec::new(&env));

        let record = AuditRecord {
            auditor,
            timestamp: env.ledger().timestamp(),
            report_hash,
        };

        audits.push_back(record);
        env.storage().instance().set(&Self::get_storage_key(), &audits);
    }

    // Get all audit records
    pub fn get_audits(env: Env) -> Vec<AuditRecord> {
        env.storage()
            .instance()
            .get(&Self::get_storage_key())
            .unwrap_or(Vec::new(&env))
    }
}