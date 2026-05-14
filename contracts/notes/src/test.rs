#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _};
use soroban_sdk::{Env, Address};

#[test]
fn test_oracle_flow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, WeatherOracle);
    let client = WeatherOracleClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let region = symbol_short!("MNL"); // Manila
    let wind_speed = 150;

    // Unauthorized call should fail
    // client.update_weather(&region, &wind_speed); 

    // Authorized call
    env.mock_all_auths();
    client.update_weather(&region, &wind_speed);

    assert_eq!(client.get_wind_speed(&region), 150);
}
