//! Benchmarking setup for pallet-toggle
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Toggle;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn change_state() {
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		change_state(RawOrigin::Signed(caller));

		assert_eq!(Something::<T>::get(), Some(true));
	}

	impl_benchmark_test_suite!(Toggle, crate::mock::new_test_ext(), crate::mock::Test);
}
