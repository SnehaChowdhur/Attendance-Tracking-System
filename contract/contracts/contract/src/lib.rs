#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, Vec};

#[contract]
pub struct AttendanceContract;

#[contractimpl]
impl AttendanceContract {

    // Mark attendance for a user
    pub fn mark_attendance(env: Env, user: Address, date: Symbol) {
        user.require_auth();

        let key = (user.clone(), date.clone());

        if env.storage().persistent().has(&key) {
            panic!("Attendance already marked");
        }

        env.storage().persistent().set(&key, &true);
    }

    // Check if attendance exists
    pub fn check_attendance(env: Env, user: Address, date: Symbol) -> bool {
        let key = (user, date);
        env.storage().persistent().get(&key).unwrap_or(false)
    }

    // Get all attendance dates for a user
    pub fn get_attendance(env: Env, user: Address) -> Vec<Symbol> {
        let mut result = Vec::new(&env);

        let storage = env.storage().persistent();

        // NOTE: Soroban doesn't support full iteration easily,
        // so in real projects you'd maintain a list separately
        // This is just a conceptual placeholder

        result
    }
}