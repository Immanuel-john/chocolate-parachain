#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{dispatch::{DispatchError, DispatchResult}, RuntimeDebug};
use frame_system::Config;
use scale_info::TypeInfo;

#[derive(
	Encode,
	Decode,
	Default,
	Eq,
	PartialEq,
	Clone,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	PartialOrd,
	Ord,
)]
pub struct User {
	pub rank_points: u32,
	pub project_id: Option<u32>,
}

/// UserIO trait for CRUD on users store
pub trait UserIO<T: Config> {
	fn get_user_by_id(id: &T::AccountId) -> Option<User>;
	fn check_owns_project(id: &T::AccountId) -> bool;
	fn check_user_exists(id: &T::AccountId) -> bool;
	/// Checks if the user exists, else creates a new user with wanted defaults.
	fn get_or_create_default(id: &T::AccountId) -> Result<User, DispatchError>;
	fn set_user(id: &T::AccountId, user: User) -> DispatchResult;
	fn update_user(id: &T::AccountId, user: User) -> DispatchResult;
}
