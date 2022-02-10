use std::cmp::{max, min};
use std::collections::BinaryHeap;
use std::convert::{TryFrom};
use std::fmt;

use itertools::Itertools;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_contract_standards::non_fungible_token::{TokenId};
use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_sdk::{AccountId, BorshStorageKey, env, log, near_bindgen, PanicOnDefault, Promise, PromiseOrValue, Timestamp};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, TreeMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::json_types::{U128, U64, ValidAccountId};
use near_sdk::serde::{Deserialize, Serialize};



near_sdk::setup_alloc!();
const MESSAGE_LIMIT: u64 = 10;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
  POSTED_MESSAGE
}

#[derive(Debug, BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
  premium: bool,
  sender: String,
  text: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  messages: Vector<PostedMessage>,
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn new() -> Self {
    Self {
      messages: Vector::new(StorageKey::POSTED_MESSAGE)
    }
  }

  #[payable]
  pub fn addMessage(&mut self, text: String) {
    self.messages.push(&PostedMessage {
      premium: env::attached_deposit() >= 10000000000000000000000 as u128,
      sender: env::predecessor_account_id(),
      text,
    });
  }

  pub fn getMessages(&self) -> Vec<PostedMessage> {
    let skip_num = if self.messages.len() >= MESSAGE_LIMIT { self.messages.len() - MESSAGE_LIMIT } else { 0 };
    let mut vec = self.messages.iter()
      .skip(skip_num as usize)
      .collect_vec();
    vec.reverse();
    vec
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use near_sdk::json_types::ValidAccountId;
  use near_sdk::test_utils::{accounts, VMContextBuilder};
  use near_sdk::{testing_env, MockedBlockchain};

  const BLOCK_START_BLOCK: u64 = 52_201_040;
  const BLOCK_START_TS: u64 = 1_624_151_503_447_000_000;

  fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(accounts(0))
      .signer_account_id(predecessor_account_id.clone())
      .signer_account_pk(b"ed25519:4ZhGmuKTfQn9ZpHCQVRwEr4JnutL8Uu3kArfxEqksfVM".to_vec())
      .predecessor_account_id(predecessor_account_id)
      .block_index(BLOCK_START_BLOCK)
      .block_timestamp(BLOCK_START_TS);
    builder
  }

  #[test]
  fn test_contract_new() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let contract = Contract::new();
    testing_env!(context.is_view(true).build());
  }

  #[test]
  fn test_add_and_view_messages() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::new();
    contract.addMessage("test".to_string());
    contract.addMessage("test1".to_string());
    println!("{:?}", contract.getMessages());
  }
}
