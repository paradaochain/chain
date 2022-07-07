#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::dao::{Dao, DaoRef};

#[ink::contract]
pub mod dao {

	use ink_prelude::{string::String, vec::Vec};
	use ink_storage::{
		traits::{PackedLayout, SpreadAllocate, SpreadLayout},
		Mapping,
	};

	use ink_lang::utils::initialize_contract;
	use ink_storage::traits::KeyPtr;

	pub type ProposalId = u32;
	/// Number of blocks until proposal expires from the proposed block
	const EXPIRATION_BLOCK_FROM_NOW: BlockNumber = 250;

	/// Total member div this number as threshold
	const PROPOSAL_THRESHOLD_DIV: u32 = 2;

	/// Errors that can occur upon calling this contract.
	#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
	#[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
	pub enum Error {
		NotEnoughMembers,
		ThresholdError,
		Overflow,
	}

	/// A Transaction is what `Proposers` can submit for voting.
	/// If votes pass a threshold, it will be executed by the DAO.
	/// Note: Struct from ink repo: multisig example
	#[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout, Clone, Debug)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
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

	#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug, SpreadAllocate)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
	pub struct Votes {
		pub yes: u32,
		pub no: u32,
	}

	#[derive(scale::Encode, scale::Decode, PackedLayout, SpreadLayout, Debug)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
	pub struct Proposal {
		pub title: String,
		pub proposer: AccountId,
		pub expires: BlockNumber,
		pub tx: Transaction,
		pub status: ProposalStatus,
		/// Number of votes required to pass = 0.5 total voters the time this was proposed
		pub threshold: u32,
		pub votes: Votes,
	}

	impl Proposal {
		pub fn new(
			title: String,
			proposer: AccountId,
			current_block: BlockNumber,
			tx: Transaction,
			members_count: u32,
		) -> Result<Self, Error> {
			let threshold =
				members_count.checked_div(PROPOSAL_THRESHOLD_DIV).ok_or(Error::ThresholdError)?;
			let expires =
				current_block.checked_add(EXPIRATION_BLOCK_FROM_NOW).ok_or(Error::Overflow)?;
			Ok(Self {
				title,
				proposer,
				expires,
				tx,
				status: ProposalStatus::Voting,
				threshold,
				votes: Votes { yes: 1, no: 0 },
			})
		}

		pub fn update_status(&mut self, current_block_num: BlockNumber, executed: bool) {
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
			self.status == ProposalStatus::Passed
		}
	}

	#[derive(
		scale::Encode, scale::Decode, Clone, Copy, SpreadLayout, PackedLayout, Debug, PartialEq,
	)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
	pub enum DaoType {
		Fanclub,
		Collab,
	}

	// TODO impl
	impl SpreadAllocate for DaoType {
		#[inline]
		fn allocate_spread(ptr: &mut KeyPtr) -> Self {
			ptr.advance_by(<BlockNumber>::FOOTPRINT * 2);
			Self::Fanclub
		}
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
	#[derive(SpreadAllocate)]
	pub struct Dao {
		/// name of dao
		name: String,
		/// Governance type
		ty: DaoType,
		/// min fee to join
		fee: Balance,
		/// Members list
		members: Mapping<AccountId, Role>,
		/// Members count
		member_count: u32,
		/// Current proposals
		proposals: Mapping<u32, Proposal>,
		/// total number of proposals
		next_proposal_id: u32,
		/// Proposal Id and its Voting status
		votes: Mapping<(u32, AccountId), bool>,
	}

	#[derive(scale::Encode, scale::Decode, Debug)]
	#[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout))]
	pub struct Info {
		name: String,
		ty: DaoType,
		fee: Balance,
	}

	impl Dao {
		#[ink(constructor)]
		pub fn new(name: String, ty: DaoType, fee: Balance, stars: Option<Vec<AccountId>>) -> Self {
			initialize_contract(|c: &mut Self| {
				c.name = name;
				c.ty = ty;
				c.fee = fee;
				if let Some(s) = stars {
					if ty == DaoType::Fanclub {
						for each in s {
							c.members.insert(each, &Role::Star);
						}
					}
				}
			})
		}

		/// Returns some useful info for the DAO
		#[ink(message)]
		pub fn info(&self) -> Info {
			Info { name: self.name.clone(), ty: self.ty, fee: self.fee }
		}

		/// Return stars
		#[ink(message)]
		pub fn role_of(&self, member: AccountId) -> Option<Role> {
			self.members.get(member)
		}

		/// Return total number of members
		#[ink(message)]
		pub fn total_members(&self) -> u32 {
			self.member_count
		}

		/// Return total proposals
		#[ink(message)]
		pub fn total_proposals(&self) -> u32 {
			self.next_proposal_id
		}

		/// Joing a DAO as a member
		#[ink(message, payable)]
		pub fn join(&mut self) {
			let caller = self.env().caller();
			assert!(self.env().transferred_value() >= self.fee);
			self.members.insert(caller, &Role::Member);
			let count = self.member_count;
			self.member_count = count.checked_add(1).expect("Overflow");
		}

		#[ink(message)]
		pub fn propose(&mut self, proposal_tx: Transaction, title: String) -> u32 {
			self.ensure_caller_is_member();
			let pid = self.next_proposal_id;
			let proposer = self.env().caller();
			let proposal = Proposal::new(
				title,
				proposer,
				self.env().block_number(),
				proposal_tx,
				self.member_count,
			)
			.unwrap_or_else(|error| panic!("failed at create proposal {:?}", error));

			self.proposals.insert(pid, &proposal);

			self.votes.insert((pid, proposer), &true);
			self.next_proposal_id = pid.checked_add(1).expect("Overflow");
			pid
		}

		// Helpers
		/// Panic if the sender is not self
		/// Usually used to promote members
		fn ensure_from_dao(&self) {
			assert_eq!(self.env().caller(), self.env().account_id());
		}

		fn ensure_caller_is_member(&self) {
			assert!(self.members.contains(self.env().caller()));
		}
	}

	/// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
	/// module and test functions are marked with a `#[test]` attribute.
	/// The below code is technically just normal Rust code.
	#[cfg(test)]
	mod tests {
		/// Imports all the definitions from the outer scope so we can use them here.
		use super::*;

		use ink_env::test;
		/// Imports `ink_lang` so we can use `#[ink::test]`.
		use ink_lang as ink;

		fn default_accounts() -> test::DefaultAccounts<Environment> {
			ink_env::test::default_accounts::<Environment>()
		}

		fn create_fanclub_dao(stars: Vec<AccountId>) -> Dao {
			Dao::new(String::from("newDAO"), DaoType::Fanclub, 2, Some(stars))
		}

		/// We test a simple use case of our contract.
		#[ink::test]
		fn create_dao_works() {
			let test_accounts = default_accounts();
			let dao = create_fanclub_dao(vec![test_accounts.alice, test_accounts.bob]);
			assert_eq!(dao.info().name, "newDAO");
			assert_eq!(dao.info().ty, DaoType::Fanclub);
		}

		//		#[ink::test]
		//		fn proposal_works() {
		//			let test_accounts = default_accounts();
		//			let dao =  create_fanclub_dao(vec![test_accounts.alice, test_accounts.bob])
		//		}
	}
}
