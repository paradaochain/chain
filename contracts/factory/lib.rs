#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod factory {
	use dao::{dao::{DaoType, CreateDaoMessage}, DaoRef};
	use ink_storage::{
		traits::{PackedLayout, SpreadAllocate, SpreadLayout},
		Mapping,
	};

	use ink_lang::utils::initialize_contract;
	use ink_prelude::{string::String, vec::Vec};

	#[ink(storage)]
	#[derive(SpreadAllocate)]
	pub struct Factory {
		next_index: u32,
		dao_contract_hash: Hash,
		daos: Vec<AccountId>,
	}

	impl Factory {
		#[ink(constructor)]
		pub fn new(next_index: u32, dao_contract_hash: Hash) -> Self {
			initialize_contract(|contract: &mut Self| {
				contract.next_index = next_index;
				contract.dao_contract_hash = dao_contract_hash;
			})
		}

		/// Creates a new dao
		#[ink(message)]
		pub fn create_dao(
			&mut self,
			info: CreateDaoMessage,
			stars: Option<Vec<AccountId>>
		) {
			ink_env::debug_println!("create DAO at {}", Self::env().block_number());

			let new_dao = DaoRef::new(info, stars)
				.endowment(0)
				.code_hash(self.dao_contract_hash)
				.salt_bytes(self.next_index.to_le_bytes())
				.params();

			let addr = self.env().instantiate_contract(&new_dao).unwrap_or_else(|error| {
				ink_env::debug_println!("ERROR at createing DAO {:?}", error);
				panic!("failed at instantiating the DAO contract: {:?}", error)
			});

			self.daos.push(addr);
			self.next_index += 1;
		}

		/// Simply returns the number of daos created
		#[ink(message)]
		pub fn get_next_index(&self) -> u32 {
			self.next_index
		}

		/// The list of all dao account addresses
		#[ink(message)]
		pub fn get_daos(&self) -> Vec<AccountId> {
			// TODO: pagination
			let mut output = Vec::new();
			for dao in &self.daos {
				output.push(*dao);
			}
			output
		}
	}

	// #[cfg(test)]
	// mod tests {
	// 	/// Imports all the definitions from the outer scope so we can use them here.
	// 	use super::*;

	// 	use ink_env::{test, Hash};
	// 	/// Imports `ink_lang` so we can use `#[ink::test]`.
	// 	use ink_lang as ink;

	// 	fn default_accounts() -> test::DefaultAccounts<Environment> {
	// 		ink_env::test::default_accounts::<Environment>()
	// 	}

	// 	/// We test a simple use case of our contract.
	// 	#[ink::test]
	// 	fn it_works() {
	// 		let hash: Hash = [0; 32].try_into().unwrap();
	// 		let test_accounts = default_accounts();
	// 		let mut factory = Factory::new(0, hash);
	// 		assert_eq!(factory.get_next_index(), 0);
	// 		factory.create_dao(String::from("newDAO"), 0, Some(vec![test_accounts.alice]), 12);
	// 		assert_eq!(factory.get_next_index(), 1);
	// 	}
	// }
}
