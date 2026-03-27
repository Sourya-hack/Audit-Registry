#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec, String, Symbol};

#[derive(Clone)]
#[contracttype]
pub struct Audit {
    pub auditor: String,
    pub report_hash: String,
}

#[contract]
pub struct AuditRegistry;

#[contractimpl]
impl AuditRegistry {

    // Add audit record
    pub fn add_audit(env: Env, auditor: String, report_hash: String) {
        let key = Symbol::new(&env, "AUDITS");

        let mut audits: Vec<Audit> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        audits.push_back(Audit {
            auditor,
            report_hash,
        });

        env.storage().instance().set(&key, &audits);
    }

    // Get all audits
    pub fn get_audits(env: Env) -> Vec<Audit> {
        let key = Symbol::new(&env, "AUDITS");

        env.storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }
}