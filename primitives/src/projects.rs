#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	dispatch::DispatchResult, sp_runtime::traits::{AtLeast32BitUnsigned,Zero}, traits::Get, BoundedVec,
	RuntimeDebug,
};
use frame_system::Config;
use scale_info::TypeInfo;
/// A simple u32
pub type ProjectID = u32;
/// Index for reviews , use to link to project
pub type ReviewID = u64;
use codec::{Decode, Encode, MaxEncodedLen};

#[derive(
	Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen, PartialOrd, Ord,
)]
pub struct Review<UserID, StringLen>
where
	StringLen: Get<u32>,
{
	pub proposal_status: ProposalStatus<StringLen>,
	pub user_id: UserID,
	pub content: BoundedVec<u8, StringLen>,
	pub project_id: ProjectID,
	/// A snapshot of the user's rank at the time of review
	pub point_snapshot: u32,
	/// Score of a review
	pub review_score: u8
}

/// The metadata of a project.
type MetaData<StringLen> = BoundedVec<u8, StringLen>;

#[cfg(feature = "std")]
pub use serde::{Deserialize, Serialize};

/// The status of the proposal
#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	PartialOrd,
	Ord,
)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
pub enum Status {
	///Proposal created
	Proposed,
	/// Proposal accepted
	Accepted,
	/// Proposal rejected
	Rejected,
}
/// Reason for the current status - Required for rejected proposal.
#[derive(
	Encode, Decode, Eq, PartialEq, Clone, RuntimeDebug, TypeInfo, MaxEncodedLen, PartialOrd, Ord,
)]
#[derive(Deserialize, Serialize)]
pub enum Reason<StringLen>
where
StringLen: Get<u32>,
{
	/// Custom reason to encapsulate further things like marketCap and other details
	Other(BoundedVec<u8, StringLen>),
	/// Negative lenient - base conditions for project missing or review lacking detail
	InsufficientMetaData,
	/// Negative harsh, project or review is malicious
	Malicious,
	/// Positive neutral, covers rank up to accepted.
	PassedRequirements,
}
/// The status of a proposal sent to the council from here.
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
pub struct ProposalStatus<StringLen>
where
StringLen: Get<u32>,

{
	pub status: Status,
	pub reason: Reason<StringLen>,
}
/// Default status - storage req
impl Default for Status {
	fn default() -> Self {
		Status::Proposed
	}
}
/// Default reason - storage req
impl<StringLen> Default for Reason<StringLen>
where
	StringLen: Get<u32>,
{
	fn default() -> Self {
		Reason::PassedRequirements
	}
}
/// The project structure.
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
pub struct Project<UserID, Balance, StringLen>
where
	Balance: AtLeast32BitUnsigned,
	StringLen: Get<u32>,
{
	/// The owner of the project
	pub owner_id: UserID,
	/// A bool that allows for simple allocation of the unique chocolate badge. NFT?? (default: false)
	badge: Option<bool>,
	/// Project metadata - req - default some .
	metadata: MetaData<StringLen>,
	/// the status of the project's proposal in the council - default proposed.
	pub proposal_status: ProposalStatus<StringLen>,
	/// A reward value for the project ---------_switch to idea of named reserve hash - (default: Reward).
	pub reward: Balance,
	/// A sum of all the points of users who wrote a review for the project. Saturate when u32::MAX.
	pub total_user_scores: u32,
	/// The total review scores for a project
	pub total_review_score: u64,
	/// The number of reviews submitted
	pub number_of_reviews: u32
}

impl<UserID, Balance, StringLen> Project<UserID, Balance, StringLen>
where
	Balance: AtLeast32BitUnsigned,
	StringLen: Get<u32>,
{
	///  Set useful defaults.
	///  Initialises a project with defaults on everything except id and metadata
	pub fn new(owner_id: UserID, metadata: MetaData<StringLen>) -> Self {
		Project {
			owner_id,
			badge: Option::None,
			metadata,
			reward: Zero::zero(),
			proposal_status: ProposalStatus {
				status: Default::default(),
				reason: Default::default()
			},
			total_user_scores: Zero::zero(),
			number_of_reviews: Zero::zero(),
			total_review_score: Zero::zero()
		}
	}
}
/// A trait that allows project to:
/// - reserve some token for rewarding its reviewers.
pub trait ProjectIO<T: Config> {
	type UserID;
	type Balance: AtLeast32BitUnsigned;
	type StringLimit: Get<u32>;
	/// Checks:
	/// If the projects' reward value reflects what is reserved, excluding existential value
	fn check_reward(
		project: &Project<Self::UserID, Self::Balance, Self::StringLimit>,
	) -> DispatchResult;
	/// Check if the project owner can offer up hardcoded amount as init.
	fn can_reward(project: &Self::UserID) -> bool;
	/// Reserve an initial amount for use as reward
	fn reserve_reward(
		project: &mut Project<Self::UserID, Self::Balance, Self::StringLimit>,
	) -> DispatchResult;
	/// Reward the user with an amount and effect edits on the struct level. (Exposes amount in free balance for next step (transfer))
	/// Assumed to be executed right before the final balance transfer
	/// # Note: 
	/// If any failure happens after, reward may be lost.
	fn reward(
		project: &mut Project<Self::UserID, Self::Balance, Self::StringLimit>,
		amount: Self::Balance,
	) -> DispatchResult;
}
/// Easy way of differentaiting the two. We'll need this.
#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	PartialOrd,
	Ord,
)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
pub enum EntityKind {
	Project,
	User,
}
