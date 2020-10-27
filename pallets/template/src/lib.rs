#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The accumulation item.
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Accumulation {
	/// Current count of the accumulation.
	pub current_count: u128,
	/// Increment for each dispatch call.
	pub increment_per_call: u128,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}


decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		pub Acc get(fn acc): Accumulation;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		Accumulated(u128, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		AccumulationOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 100]
		pub fn set_increment(origin, increment: u128) -> dispatch::DispatchResult {
			let _ = ensure_signed(origin)?;

			let mut acc = Self::acc();
			acc.increment_per_call = increment;

			Acc::put(acc);

			Ok(())
		}

		#[weight = 100]
		pub fn accumulate(origin) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			let mut acc = Self::acc();
			acc.current_count = acc.current_count.checked_add(acc.increment_per_call).ok_or(Error::<T>::AccumulationOverflow)?;

			Acc::put(acc.clone());

			Self::deposit_event(RawEvent::Accumulated(acc.current_count, who));

			Ok(())
		}
	}
}
