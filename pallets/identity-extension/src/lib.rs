#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::boxed::Box;

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config + pallet_identity::Config {}

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(_);

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_identity(
			origin: OriginFor<T>,
			info: Box<pallet_identity::IdentityInfo<T::MaxAdditionalFields>>,
		) -> DispatchResultWithPostInfo {
			let mut info = info;

			info.additional
				.try_push((
					pallet_identity::Data::Raw(b"created_by".to_vec().try_into().unwrap()),
					pallet_identity::Data::Raw(b"identity_extension".to_vec().try_into().unwrap()),
				))
				.unwrap();

			pallet_identity::Pallet::<T>::set_identity(origin, info)
		}
	}
}
