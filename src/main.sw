contract;
use std::{
    auth::{
        AuthError,
        msg_sender,
    },
    call_frames::{
        contract_id,
        msg_asset_id,
    },
    context::{
        balance_of,
        msg_amount,
    },
    revert::require,
    token::transfer_to_address,
};

storage {
    deposits: StorageMap<(Address, ContractId), u64> = StorageMap {},
}

enum Error {
    InsufficientBalance: (),
}

abi Moneybox {
    #[storage(write, read)]
    fn deposit();

    #[storage(write, read)]
    fn withdraw(asset_id: ContractId, amount: u64);

    #[storage(read)]
    fn balance(address: Address, asset_id: ContractId) -> u64;

    fn total_balance_of(asset_id: ContractId) -> u64;
}

fn get_msg_sender_address_or_panic() -> Address {
    let sender: Result<Identity, AuthError> = msg_sender();
    if let Identity::Address(address) = sender.unwrap() {
        address
    } else {
        revert(0);
    }
}

#[storage(read)]
fn balance_internal(address: Address, asset_id: ContractId) -> u64 {
    let key = (address, asset_id);
    storage.deposits.get(key)
}

impl Moneybox for Contract {
    #[storage(write, read)]
    fn deposit() {
        let amount = msg_amount();
        let asset_id = msg_asset_id();
        let address = get_msg_sender_address_or_panic();

        let key = (address, asset_id);
        let amount = amount + storage.deposits.get(key);
        storage.deposits.insert(key, amount);
    }

    #[storage(write, read)]
    fn withdraw(asset_id: ContractId, amount: u64) {
        let address = get_msg_sender_address_or_panic();
        let balance = balance_internal(address, asset_id);
        require(balance >= amount, Error::InsufficientBalance);
        transfer_to_address(amount, asset_id, address);
        let amount_after = balance - amount;
        let key = (address, asset_id);
        if amount_after > 0 {
            storage.deposits.insert(key, amount_after);
        } else {
            storage.deposits.insert(key, 0);
        }
    }

    #[storage(read)]
    fn balance(address: Address, asset_id: ContractId) -> u64 {
        balance_internal(address, asset_id)
    }

    fn total_balance_of(asset_id: ContractId) -> u64 {
        balance_of(contract_id(), asset_id)
    }
}
