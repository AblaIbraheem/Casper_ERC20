#![no_main]
#![no_std]

extern crate alloc;

use alloc::string::String;

use casper_types::{Key, U256};
use contract_interface::contract_interface;
use contract_utils::{ContractContext, OnChainContractStorage};
// use erc20::data::{Proposal, ProposalId};
use erc20::{self, Backer, Proposal, ProposalId, ERC20};

#[derive(Default)]
struct Token(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for Token {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl ERC20<OnChainContractStorage> for Token {}

impl Token {
    fn constructor(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256) {
        ERC20::init(self, name, symbol, decimals);
        ERC20::mint(self, self.get_caller(), initial_supply);
    }
}

#[contract_interface(Token)]
trait ERC20Interface {
    fn constructor(&mut self, name: String, symbol: String, decimals: u8, initial_supply: U256);
    fn transfer(&mut self, recipient: Key, amount: U256);
    fn add_stakeholder(&mut self, recipient: Key, amount: U256);
    fn add_backer(&mut self, proposal_id: ProposalId, backer: Backer);
    fn add_proposal(&mut self, proposal_id: ProposalId, proposal: Proposal);
    fn get_proposal(&mut self, proposal_id: ProposalId) -> Option<Proposal>;
    fn remove_stakeholder(&mut self, recipient: Key);
    fn remove_backer(&mut self, proposal_id: ProposalId);
    fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256);
    fn approve(&mut self, spender: Key, amount: U256);
    fn balance_of(&mut self, owner: Key) -> U256;
    fn stake_of(&mut self, owner: Key) -> U256;
    fn back_of(&mut self, proposal_id: ProposalId) -> Option<Backer>;
    fn allowance(&mut self, owner: Key, spender: Key) -> U256;
    fn total_supply(&mut self) -> U256;
}
