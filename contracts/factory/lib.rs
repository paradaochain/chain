#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod factory {
	use dao::{
		dao::{DaoType, Role},
		DaoRef,
	};
	use ink_storage::traits::SpreadAllocate;

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
			name: String,
			ty: u32,
			joining_fee: Balance,
			init_members: Vec<(AccountId, String, Role)>,
			salt: u32,
		) {
			let daotype = if ty == 0 { DaoType::Fanclub } else { DaoType::Collab };
			ink_env::debug_println!("create DAO at {}", Self::env().block_number());

			let new_dao = DaoRef::new(name, daotype, joining_fee, init_members)
				.endowment(0)
				.code_hash(self.dao_contract_hash)
				.salt_bytes(salt.to_le_bytes())
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
}
