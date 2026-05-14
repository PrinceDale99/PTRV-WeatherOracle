#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, log};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Policy(Address), // Stores policy details for a specific farmer
    Oracle,          // The authorized weather data provider
    Stablecoin,      // The PHP-stablecoin asset address
}

#[derive(Clone)]
#[contracttype]
pub struct Policy {
    pub region: Symbol,
    pub premium: i128,
    pub payout_amount: i128,
    pub threshold_kmh: u32,
    pub is_active: bool,
}

#[contract]
pub struct TyphoonVault;

#[contractimpl]
impl TyphoonVault {
    // Initialize the contract with the authorized oracle and stablecoin address
    pub fn initialize(env: Env, oracle: Address, stablecoin: Address) {
        env.storage().instance().set(&DataKey::Oracle, &oracle);
        env.storage().instance().set(&DataKey::Stablecoin, &stablecoin);
    }

    // Farmer subscribes by depositing a micro-premium
    pub fn subscribe(env: Env, farmer: Address, region: Symbol, premium: i128) {
        farmer.require_auth();
        
        let policy = Policy {
            region,
            premium,
            payout_amount: premium * 10, // 10x leverage for disaster relief
            threshold_kmh: 150,          // Signal No. 3+ threshold
            is_active: true,
        };

        env.storage().persistent().set(&DataKey::Policy(farmer.clone()), &policy);
        log!(&env, "Farmer subscribed to policy", farmer);
    }

    // Triggered by the Oracle when weather data confirms a breach
    pub fn trigger_payout(env: Env, oracle: Address, farmer: Address, wind_speed: u32) {
        oracle.require_auth();
        
        let authorized_oracle: Address = env.storage().instance().get(&DataKey::Oracle).unwrap();
        if oracle != authorized_oracle {
            panic!("Unauthorized oracle");
        }

        let mut policy: Policy = env.storage().persistent().get(&DataKey::Policy(farmer.clone())).unwrap();
        
        if policy.is_active && wind_speed >= policy.threshold_kmh {
            policy.is_active = false; // Prevent double payout
            env.storage().persistent().set(&DataKey::Policy(farmer.clone()), &policy);
            
            // In a real scenario, a Cross-Contract Call to the Token contract would happen here
            log!(&env, "Threshold breached. Payout triggered for:", farmer);
        } else {
            panic!("Threshold not met or policy inactive");
        }
    }
}
