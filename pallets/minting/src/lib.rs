#![cfg_attr(not(feature = "std"), no_std)]


/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;




#[frame_support::pallet]
pub mod pallet {
	use frame_support::{traits::{Currency, OnUnbalanced, Imbalance},sp_runtime::traits::Zero, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	use super::*;
	
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Currency
		type Currency: Currency<Self::AccountId>;
		/// * Treasury outlet: A type with bounds to move slashed funds to the treasury.
		type TreasuryOutlet: OnUnbalanced<NegativeImbalanceOf<Self>>;
		///  Origins that must approve to use the pallet - Should be implemented properly by provider.
		type ApproveOrigin: EnsureOrigin<Self::Origin>;
	}
	
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	
	pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Parameters. [Amount]
		Minted(BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Mint Must be called from a root or equivalent origin
		InvalidOrigin,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

 	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn mint(origin: OriginFor<T>, x: BalanceOf<T>) -> DispatchResult {
			// call its ensure origin - doesn't return origin. Only checks
			T::ApproveOrigin::ensure_origin(origin)?;
			let imbalance = T::Currency::issue(x);
			let minted = imbalance.peek();
			Self::do_mint(imbalance);
			Self::deposit_event(Event::Minted(minted));
			Ok(())
		}
	}
	impl<T: Config> Pallet<T> {
		/// Function to take negative imbalance to the treasury, expected to be called after creating one e.g through T::Currency::issue()
		pub fn do_mint(amount: NegativeImbalanceOf<T>) {
			T::TreasuryOutlet::on_unbalanced(amount);
		}
	}


	/// Genesis config for the minting pallet. Use to mint an initial amount to treasury
	/// E.g Use: Token sales.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		/// Amount to mint
		pub init_mint: BalanceOf<T>,
	}

	/// By default, nothing.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { init_mint: Zero::zero() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// Repeat mint call, without origin check
			let imbalance = T::Currency::issue(self.init_mint);
			let minted = imbalance.peek();
			<Pallet<T>>::do_mint(imbalance);
			<Pallet<T>>::deposit_event(Event::Minted(minted));
		}
	}
}
