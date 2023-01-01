use crate::utils::number_utils::parse_units;
use fuels::prelude::*;
use rand::prelude::Rng;

abigen!(MoneyBox, "out/debug/money-box-abi.json");

abigen!(
    TokenContract,
    "tests/artefacts/token/token_contract-abi.json"
);

pub mod wallet_abi_calls {
    use fuels::contract::call_response::FuelCallResponse;

    use super::*;

    pub async fn balance(
        contract: &MoneyBox, addresss: Address, asset: ContractId
    ) -> Result<FuelCallResponse<u64>, Error> {
        contract.methods().balance(addresss,asset).simulate().await
    }
}

pub struct DeployTokenConfig {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub mint_amount: u64,
}

pub async fn init_wallet() -> WalletUnlocked {
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(1),             /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;
    wallets.pop().unwrap()
}

pub async fn get_money_box_instance(wallet: &WalletUnlocked) -> MoneyBox {
    let id = Contract::deploy(
        "./out/debug/money-box.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::default(),
    )
    .await
    .unwrap();

    MoneyBox::new(id, wallet.clone())
}

pub async fn get_token_contract_instance(
    wallet: &WalletUnlocked,
    deploy_config: &DeployTokenConfig,
) -> TokenContract {
    let mut name = deploy_config.name.clone();
    let mut symbol = deploy_config.symbol.clone();
    let decimals = deploy_config.decimals;

    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();

    let id = Contract::deploy_with_parameters(
        "./tests/artefacts/token/token_contract.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::default(),
        Salt::from(salt),
    )
    .await
    .unwrap();

    let instance = TokenContract::new(id, wallet.clone());
    let methods = instance.methods();

    let mint_amount = parse_units(deploy_config.mint_amount, decimals);
    name.push_str(" ".repeat(32 - deploy_config.name.len()).as_str());
    symbol.push_str(" ".repeat(8 - deploy_config.symbol.len()).as_str());

    let config: token_contract_mod::Config = token_contract_mod::Config {
        name: fuels::core::types::SizedAsciiString::<32>::new(name).unwrap(),
        symbol: fuels::core::types::SizedAsciiString::<8>::new(symbol).unwrap(),
        decimals,
    };

    let _res = methods
        .initialize(config, mint_amount, Address::from(wallet.address()))
        .call()
        .await;
    let _res = methods.mint().append_variable_outputs(1).call().await;

    instance
}

pub async fn print_balances(wallet: &WalletUnlocked) {
    let balances = wallet.get_balances().await.unwrap();
    println!("{:#?}\n", balances);
}
