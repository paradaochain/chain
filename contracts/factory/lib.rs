#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod factory {
	use dao::DaoRef;
	use ink_prelude::string::String;

	#[ink(storage)]
	pub struct Factory {
		next_index: u32,
		dao_contract_hash: Hash,
	}

	impl Factory {
		#[ink(constructor)]
		pub fn new(next_index: u32, dao_contract_hash: Hash) -> Self {
			Self { next_index, dao_contract_hash }
		}

		/// Creates a new dao
		#[ink(message)]
		pub fn create_dao(&mut self, name: String, salt: u32) {
			self.next_index += 1;

			let _dao = DaoRef::new(name)
				.endowment(0)
				.code_hash(self.dao_contract_hash)
				.salt_bytes(salt.to_le_bytes())
				.instantiate()
				.unwrap_or_else(|error| {
					panic!("failed at instantiating the DAO contract: {:?}", error)
				});
			self.next_index += 1;
		}

		/// Simply returns the current value of our `bool`.
		#[ink(message)]
		pub fn get_next_index(&self) -> u32 {
			self.next_index
		}
	}

	/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
	/// module and test functions are marked with a `#[test]` attribute.
	/// The below code is technically just normal Rust code.
	#[cfg(test)]
	mod tests {
		/// Imports all the definitions from the outer scope so we can use them here.
		use super::*;

		use ink_env::Hash;
		/// Imports `ink_lang` so we can use `#[ink::test]`.
		use ink_lang as ink;

		/// We test a simple use case of our contract.
		#[ink::test]
		fn it_works() {
			let hash: Hash = [0; 32].try_into().unwrap();
			let mut factory = Factory::new(0, hash);
			assert_eq!(factory.get_next_index(), 0);
			factory.create_dao("new".into());
			assert_eq!(factory.get_next_index(), 1);
		}
	}
}
