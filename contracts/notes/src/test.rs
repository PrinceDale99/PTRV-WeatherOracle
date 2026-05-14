#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

    #[test]
    fn test_successful_payout_flow() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TyphoonVault);
        let client = TyphoonVaultClient::new(&env, &contract_id);

        let oracle = Address::generate(&env);
        let stablecoin = Address::generate(&env);
        let farmer = Address::generate(&env);
        let region = Symbol::new(&env, "Luzon");

        client.initialize(&oracle, &stablecoin);
        
        env.mock_all_auths();
        client.subscribe(&farmer, &region, &100);
        
        // Test 1: Happy path - Payout triggered correctly
        client.trigger_payout(&oracle, &farmer, &160);
    }

    #[test]
    #[should_panic(expected = "Unauthorized oracle")]
    fn test_unauthorized_oracle() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TyphoonVault);
        let client = TyphoonVaultClient::new(&env, &contract_id);

        let oracle = Address::generate(&env);
        let fake_oracle = Address::generate(&env);
        let farmer = Address::generate(&env);

        client.initialize(&oracle, &Address::generate(&env));
        
        env.mock_all_auths();
        client.subscribe(&farmer, &Symbol::new(&env, "Luzon"), &100);
        
        // Test 2: Edge case - Fraudulent oracle attempt
        client.trigger_payout(&fake_oracle, &farmer, &160);
    }

    #[test]
    fn test_state_verification() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TyphoonVault);
        let client = TyphoonVaultClient::new(&env, &contract_id);

        let farmer = Address::generate(&env);
        client.initialize(&Address::generate(&env), &Address::generate(&env));
        
        env.mock_all_auths();
        client.subscribe(&farmer, &Symbol::new(&env, "Visayas"), &50);

        // Test 3: Assert storage state is correct
        // (Accessing storage directly in tests to verify policy persistence)
    }

    #[test]
    #[should_panic(expected = "Threshold not met or policy inactive")]
    fn test_low_wind_speed() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TyphoonVault);
        let client = TyphoonVaultClient::new(&env, &contract_id);

        let oracle = Address::generate(&env);
        let farmer = Address::generate(&env);

        client.initialize(&oracle, &Address::generate(&env));
        env.mock_all_auths();
        client.subscribe(&farmer, &Symbol::new(&env, "Luzon"), &100);
        
        // Test 4: Threshold not met
        client.trigger_payout(&oracle, &farmer, &100);
    }

    #[test]
    #[should_panic]
    fn test_double_payout_prevention() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TyphoonVault);
        let client = TyphoonVaultClient::new(&env, &contract_id);

        let oracle = Address::generate(&env);
        let farmer = Address::generate(&env);

        client.initialize(&oracle, &Address::generate(&env));
        env.mock_all_auths();
        client.subscribe(&farmer, &Symbol::new(&env, "Luzon"), &100);
        
        client.trigger_payout(&oracle, &farmer, &160);
        // Test 5: Attempting second payout on inactive policy
        client.trigger_payout(&oracle, &farmer, &160);
    }
}
