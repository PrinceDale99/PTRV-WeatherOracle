#![no_std]
mod test;
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, symbol_short};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Admin,
    Weather(Symbol),
}

#[contract]
pub struct WeatherOracle;

#[contractimpl]
impl WeatherOracle {
    /// Initialize the contract with an admin address.
    /// Only the admin can update weather data.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Update the wind speed for a specific region.
    /// Requires authorization from the admin.
    pub fn update_weather(env: Env, region: Symbol, wind_speed: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        admin.require_auth();

        env.storage().instance().set(&DataKey::Weather(region), &wind_speed);
    }

    /// Get the wind speed for a specific region.
    /// Returns 0 if no data is found.
    pub fn get_wind_speed(env: Env, region: Symbol) -> u32 {
        env.storage().instance().get(&DataKey::Weather(region)).unwrap_or(0)
    }

    /// Update the admin address.
    pub fn set_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
}
