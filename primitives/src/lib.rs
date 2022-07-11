#![cfg_attr(not(feature = "std"), no_std)]

// TL-imports to make mod lives easier.
use frame_support::{
	dispatch::DispatchResult,
	sp_runtime::traits::Zero,
	traits::{Get,tokens::Balance as BalanceTrait},
	BoundedVec, RuntimeDebug,
};
use frame_system::Config;
use scale_info::TypeInfo;
use codec::{Decode, Encode, MaxEncodedLen};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};


pub mod projects;
pub mod users;