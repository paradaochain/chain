#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::dao::{Dao, DaoRef};

#[ink::contract]
pub mod dao {

	use ink_prelude::string::String;
	use ink_storage::{
		traits::{PackedLayout, SpreadAllocate, SpreadLayout},
		Mapping,
	};

	/// A Transaction is what `Proposers` can submit for voting.
	/// If votes pass a threshold, it will be executed by the DAO.
	/// Note: Struct from ink repo: multisig example
	#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Clone)]
	#[cfg_attr(
		feature = "std",
		derive(Debug, PartialEq, Eq, scale_info::TypeInfo, ink_storage::traits::StorageLayout)
	)]
	pub struct Transaction {
		/// The `AccountId` of the contract that is called in this transaction.
		pub callee: AccountId,
		/// The selector bytes that identifies the function of the callee that should be called.
		pub selector: [u8; 4],
		/// The SCALE encoded parameters that are passed to the called function.
		pub input: Vec<u8>,
		/// The amount of chain balance that is transferred to the callee.
		pub transferred_value: Balance,
		/// Gas limit for the execution of the call.
		pub gas_limit: u64,
	}

	#[derive(
		scale::Encode, scale::Decode, Clone, Copy, SpreadLayout, PackedLayout, Debug, PartialEq,
	)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
	pub enum ProposalStatus {
		Voting,
		Expired,
		Rejected,
		Passed,
		Executed,
	}

	#[derive(SpreadLayout, Debug, SpreadAllocate)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
	pub struct Votes {
		pub ballots: Mapping<AccountId, bool>,
		pub yes: u32,
		pub no: u32,
	}

	#[derive(SpreadLayout, Debug)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
	pub struct Proposal {
		pub title: String,
		pub proposer: AccountId,
		pub expires: BlockNumber,
		pub tx: Transaction,
		pub status: ProposalStatus,
		/// Number of votes required to pass = 0.5 total voters the time this was proposed
		pub threshold: u32,
		/// Votes recorded for this proposal
		pub votes: Votes,
	}

	impl Proposal {
		pub fn update_status(&self, current_block_num: BlockNumber, executed: bool) {
			if executed {
				self.status = ProposalStatus::Executed;
			} else if current_block_num >= self.expires {
				self.status = ProposalStatus::Expired;
			} else if self.votes.yes >= self.threshold {
				self.status = ProposalStatus::Passed;
			} else if self.votes.no >= self.threshold {
				self.status = ProposalStatus::Rejected;
			}
		}

		pub fn can_execute(&self) -> bool {
			return self.status == ProposalStatus::Passed;
		}
	}

	#[derive(scale::Encode, scale::Decode, Clone, Copy, SpreadLayout, PackedLayout, Debug)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
	pub enum DaoType {
		Fanclub,
		Collab,
	}

	/// Roles in the DAO
	/// Star: Transfer treasury, start poll + proposal
	/// Collab: Start poll + proposal, vote
	/// Member: Vote on poll and proposal
	#[derive(scale::Encode, scale::Decode, Clone, Copy, SpreadLayout, PackedLayout, Debug)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
	pub enum Role {
		/// This is usually the content creator(s) that this DAO supports.
		/// They have full access to the DAO funds but cannot vote
		Star,
		/// This is the role that runs a DAO
		/// i.e. in Meetup, they might be volunteers for venue booking, printing marketing materials
		Collab,
		/// This is a member, participant in the DAO and can vote
		Member,
	}

	#[ink(storage)]
	pub struct Dao {
		name: String,
		ty: DaoType,
		members: Mapping<AccountId, Role>,
		proposals: Mapping<u32, Proposal>,
	}

	impl Dao {
		/// Constructor that initializes the `bool` value to the given `init_value`.
		#[ink(constructor)]
		pub fn new(name: String, ty: DaoType, admin: Option<Vec<AccountId>>) -> Self {
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
