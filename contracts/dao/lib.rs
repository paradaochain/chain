#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::dao::{Dao, DaoRef};

#[ink::contract]
pub mod dao {

	use ink_prelude::string::String;

	#[ink(storage)]
	pub struct Dao {
		name: String,
	}

	impl Dao {
		/// Constructor that initializes the `bool` value to the given `init_value`.
		#[ink(constructor)]
		pub fn new(name: String) -> Self {
			Self { name }
		}

		/// Simply returns the current value of our `bool`.
		#[ink(message)]
		pub fn name(&self) -> String {
			self.name.clone()
		}
	}

	/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
	/// module and test functions are marked with a `#[test]` attribute.
	/// The below code is technically just normal Rust code.
	#[cfg(test)]
	mod tests {
		/// Imports all the definitions from the outer scope so we can use them here.
		use super::*;

		/// Imports `ink_lang` so we can use `#[ink::test]`.
		use ink_lang as ink;

		/// We test a simple use case of our contract.
		#[ink::test]
		fn it_works() {
			let dao = Dao::new(String::from("newDAO"));
			assert_eq!(dao.name(), "newDAO");
		}
	}
}
