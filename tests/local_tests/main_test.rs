use fuels::{
    prelude::CallParameters,
    tx::{Address, AssetId, ContractId},
};

use crate::utils::{
    local_tests_utils::*,
    number_utils::{format_units, parse_units},
};

#[tokio::test]
async fn main_test() {
    //--------------- CREATE WALLET ---------------
    let wallet = init_wallet().await;
    let address = Address::from(wallet.address());
    println!("Wallet address {address}\n");

    //--------------- DEPLOY TOKEN ---------------
    let usdc_config = DeployTokenConfig {
        name: String::from("USD Coin"),
        symbol: String::from("USDC"),
        decimals: 6,
        mint_amount: 10000,
    };

    let token_instance = get_token_contract_instance(&wallet, &usdc_config).await;
    let asset_id = AssetId::from(*token_instance.get_contract_id().hash());
    let contract_asset_id = ContractId::from(token_instance.get_contract_id());

    print_balances(&wallet).await;
    let money_box = get_money_box_instance(&wallet).await;

    let methods = money_box.methods();

    let deposit_amount = parse_units(100, usdc_config.decimals);

    // //-----------------------------------------------
    // //first deposit for 100 tokens and check that balance is 100
    let call_params = CallParameters::new(Some(deposit_amount), Some(asset_id), None);
    methods
        .deposit()
        .call_params(call_params)
        .call()
        .await
        .expect("❌ first deposit failed");

    let balance = wallet_abi_calls::balance(&money_box, address, contract_asset_id)
        .await
        .unwrap();
    assert_eq!(balance.value, parse_units(100, usdc_config.decimals));

    let formatted_balance = format_units(balance.value, usdc_config.decimals);
    println!(
        "✅ first deposit for 100 USDC is done and total balance is {} USDC",
        formatted_balance
    );

    // //-----------------------------------------------
    // // first withdraw 100 tokens and check that balance is 0
    let withdraw_amount = parse_units(100, usdc_config.decimals);
    let call_params = CallParameters::new(Some(withdraw_amount), Some(asset_id), None);
    methods
        .withdraw(contract_asset_id, withdraw_amount)
        .call_params(call_params)
        .estimate_tx_dependencies(None)
        .await
        .unwrap()
        .call()
        .await
        .expect("❌ first withdraw failed");

    let balance = methods
        .balance(Address::from(wallet.address()), contract_asset_id)
        .simulate()
        .await
        .unwrap()
        .value;
    assert_eq!(balance, parse_units(0, usdc_config.decimals));
    let formatted_balance = format_units(balance, usdc_config.decimals);
    println!(
        "✅ first withdraw for 100 USDC is done and total balance is {} USDC",
        formatted_balance
    );

    // //-----------------------------------------------
    // //second deposit for 50 tokens and check that balance is 50
    let deposit_amount = parse_units(50, usdc_config.decimals);
    let call_params = CallParameters::new(Some(deposit_amount), Some(asset_id), None);
    methods
        .deposit()
        .call_params(call_params)
        .call()
        .await
        .expect("❌ second deposit failed");

    let balance = methods
        .balance(Address::from(wallet.address()), contract_asset_id)
        .simulate()
        .await
        .unwrap()
        .value;
    assert_eq!(balance, parse_units(50, usdc_config.decimals));
    let formatted_balance = format_units(balance, usdc_config.decimals);
    println!(
        "✅ second deposit for 50 USDC is done and total balance is {} USDC",
        formatted_balance
    );

    // //-----------------------------------------------
    // //third deposit for 150 tokens and check that balance is 200
    let deposit_amount = parse_units(150, usdc_config.decimals);
    let call_params = CallParameters::new(Some(deposit_amount), Some(asset_id), None);
    methods
        .deposit()
        .call_params(call_params)
        .call()
        .await
        .expect("❌ third deposit failed");

    let balance = methods
        .balance(Address::from(wallet.address()), contract_asset_id)
        .simulate()
        .await
        .unwrap()
        .value;
    assert_eq!(balance, parse_units(200, usdc_config.decimals));
    let formatted_balance = format_units(balance, usdc_config.decimals);
    println!(
        "✅ third deposit for 150 USDC is done and total balance is {} USDC",
        formatted_balance
    );

    //-----------------------------------------------
    // second withdraw for 15 tokens and check that balance is 185
    let withdraw_amount = parse_units(15, usdc_config.decimals);
    let call_params = CallParameters::new(Some(withdraw_amount), Some(asset_id), None);
    methods
        .withdraw(contract_asset_id, withdraw_amount)
        .call_params(call_params)
        .estimate_tx_dependencies(None)
        .await
        .unwrap()
        .call()
        .await
        .expect("❌ first withdraw failed");

    let balance = methods
        .balance(Address::from(wallet.address()), contract_asset_id)
        .simulate()
        .await
        .unwrap()
        .value;
    assert_eq!(balance, parse_units(185, usdc_config.decimals));
    let formatted_balance = format_units(balance, usdc_config.decimals);
    println!(
        "✅ second withdraw for 200 USDC is done and total balance is {} USDC",
        formatted_balance
    );
}
