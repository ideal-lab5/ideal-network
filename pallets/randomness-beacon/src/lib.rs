/*
 * Copyright 2024 by Ideal Labs, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![cfg_attr(not(feature = "std"), no_std)]
use codec::MaxEncodedLen;

use serde::{Serialize, Deserialize};

use frame_support::{
	pallet_prelude::*,
	traits::Get,
	BoundedVec,
};

use sp_std::prelude::*;
use frame_system::pallet_prelude::*;

use codec::{Decode, Encode};
use scale_info::TypeInfo;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use etf_crypto_primitives::utils::interpolate_threshold_bls;
use w3f_bls::{DoublePublicKey, DoubleSignature, EngineBLS, Message, SerializableToBytes, TinyBLS377};
use sp_consensus_beefy_etf::{
	Commitment, ValidatorSetId, Payload, known_payloads,
};
use sha3::{Digest, Sha3_512};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

pub type OpaqueSignature = BoundedVec<u8, ConstU32<48>>;

#[derive(
	Default, Clone, Eq, PartialEq, RuntimeDebugNoBound, 
	Encode, Decode, TypeInfo, MaxEncodedLen, Serialize, Deserialize)]
pub struct PulseHeader<BN: core::fmt::Debug> {
	pub block_number: BN,
	pub hash_prev: BoundedVec<u8, ConstU32<64>>
}

#[derive(
	Default, Clone, Eq, PartialEq, RuntimeDebugNoBound, 
	Encode, Decode, TypeInfo, MaxEncodedLen, Serialize, Deserialize)]
pub struct PulseBody {
	pub double_sig: BoundedVec<u8, ConstU32<48>>,
}

#[derive(
	Default, Clone, Eq, PartialEq, RuntimeDebugNoBound, 
	Encode, Decode, TypeInfo, MaxEncodedLen, Serialize, Deserialize)]
pub struct Pulse<BN: core::fmt::Debug> {
	header: PulseHeader<BN>,
	body: PulseBody,	
}

impl<BN: core::fmt::Debug> Pulse<BN> {

	// builds the next pulse from a previous one
	pub fn build_next(
		signature: OpaqueSignature,
		block_number: BN,
		prev: Pulse<BN>,
	) -> Self {
		let mut hasher = Sha3_512::new();
		hasher.update(prev.body.double_sig.to_vec());
		let hash_prev = hasher.finalize();

		let bounded_hash = BoundedVec::<u8, ConstU32<64>>::try_from(hash_prev.to_vec())
			.expect("the hasher should work fix this later though");

		let header: PulseHeader<BN> = PulseHeader {
			block_number,
			hash_prev: bounded_hash
		};

		let body = PulseBody {
			double_sig: signature,
		};

		Pulse {
			header,
			body,
		}
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config +
		pallet_etf::Config + 
		pallet_beefy::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The maximum number of pulses to store in runtime storage
		#[pallet::constant]
		type MaxPulses: Get<u32>;
		// TODO
		// /// Weights for this pallet.
		// type WeightInfo: WeightInfo;
	}
	
	/// the chain of randomness
	#[pallet::storage]
	pub type Pulses<T: Config> = StorageValue<
		_,
		BoundedVec<Pulse<BlockNumberFor<T>>, T::MaxPulses>,
		ValueQuery,
	>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub genesis_pulse: Pulse<BlockNumberFor<T>>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { 
				genesis_pulse: Pulse::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			Pallet::<T>::initialize(&self.genesis_pulse)
				.expect("The genesis pulse must be well formatted.");
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// the origin should be unsigned
		InvalidOrigin,
		/// the signature could not be verified
		InvalidSignature,
		SignatureNotDeserializable,
		AlreadyInitialized,
		/// the bounded runtime storage has reached its limit
		PulseOverflow,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T> {
		PulseStored,
		InvalidSignatureNotStored,
	}

	/// Writes a new block from the randomness beacon into storage
	/// if it can be verified
	///
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn write_pulse(
			_origin: OriginFor<T>,
			signatures: Vec<Vec<u8>>,
			block_number: BlockNumberFor<T>,
		) -> DispatchResult {
			// do we want a signed origin? maybe...
			let round_pk_bytes: Vec<u8> = <pallet_etf::Pallet<T>>::round_pubkey().to_vec();
			let rk = DoublePublicKey::<TinyBLS377>::deserialize_compressed(
				&round_pk_bytes[..]
			).unwrap();
			let validator_set_id = <pallet_beefy::Pallet<T>>::validator_set_id();
			let _ = Self::try_add_pulse(
				signatures, 
				block_number, 
				rk, 
				validator_set_id
			)?;
			Self::deposit_event(Event::PulseStored);
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// initialize the genesis state for this pallet
	fn initialize(genesis_pulse: &Pulse<BlockNumberFor<T>>) -> Result<(), Error<T>> {
		let mut pulses = <Pulses<T>>::get();
		// check that it hasn't already been initialized
		if !pulses.is_empty() {
			return Err(Error::<T>::AlreadyInitialized);
		}
		pulses.try_push(genesis_pulse.clone())
			.map_err(|_| Error::<T>::PulseOverflow)?;
		<Pulses<T>>::put(pulses);
		Ok(())
	}

	/// add a new pulse to the hash chain
	fn try_add_pulse(
		raw_signatures: Vec<Vec<u8>>,
		block_number: BlockNumberFor<T>,
		rk: DoublePublicKey<TinyBLS377>,
		validator_set_id: ValidatorSetId,
	) -> Result<(), Error<T>> {

		// convert all sigs to DoubleSigs + verify each one manually?
		// then we can interpolate and store onchain
		// bit that seems somewhat horrible...

		let payload = Payload::from_single_entry(
			known_payloads::ETF_SIGNATURE, 
			Vec::new()
		);
		let commitment = Commitment { 
			payload, 
			block_number, 
			validator_set_id,
		};

		// panic!("{:?}", raw_signatures.len());
		let mut good_sigs = Vec::new();
		raw_signatures.iter().enumerate().for_each(|(idx, rs)| {
			// expected pubkey
			let etf_pk = <pallet_etf::Pallet<T>>::commitments()[0].encode();
			// let round_pk_bytes: Vec<u8> = <pallet_etf::Pallet<T>>::round_pubkey().to_vec();
			let pk = DoublePublicKey::<TinyBLS377>::deserialize_compressed(
				&etf_pk[..]
			).unwrap();
			if let Ok(sig) = DoubleSignature::<TinyBLS377>::from_bytes(&rs) {	
				if sig.verify(&Message::new(b"", &commitment.encode()), &pk) {
					good_sigs.push((<TinyBLS377 as EngineBLS>::Scalar::from((idx as u8) + 1), sig.0));
					// panic!("oh wow it worked");
				} 
				// else {
				// 	// panic!("unverifiable");
				// }
			} 
			// else {
			// 	// panic!("fuck");
			// }
		});

		let sig = interpolate_threshold_bls::<TinyBLS377>(good_sigs);
		let mut bytes = Vec::new();
		sig.serialize_compressed(&mut bytes).unwrap();
		let bounded_sig = 
			BoundedVec::<u8, ConstU32<48>>::try_from(bytes)
				.map_err(|_| Error::<T>::InvalidSignature)?; // shouldn't ever happen?
		let pulses = <Pulses<T>>::get();


		// if valid, then interpolate the sig
		
		// within the 'signature group', but not a double signature...
		// let sig = interpolate_threshold_bls::<TinyBLS377>(vec![
		//     (<TinyBLS377 as EngineBLS>::Scalar::from(1u8), sig_1.0),
		//     (<TinyBLS377 as EngineBLS>::Scalar::from(2u8), sig_2.0),
		//     (<TinyBLS377 as EngineBLS>::Scalar::from(3u8), sig_3.0),
		// ]);


		// note: the pulses list cannot be empty (set on genesis)
		let mut last_pulse = Pulse::default();
		if !pulses.is_empty() {
			last_pulse = pulses[pulses.len() - 1].clone();
		}
		
		let pulse = Pulse::build_next(
			bounded_sig, 
			block_number, 
			last_pulse
		);
		let mut pulses = <Pulses<T>>::get();
		pulses.try_push(pulse.clone())
			.map_err(|_| Error::<T>::PulseOverflow)?;
		<Pulses<T>>::put(pulses);
		Ok(())
		// return Ok(());


		// if let Ok(sig) = DoubleSignature::<TinyBLS377>::from_bytes(&raw_signature) {
		// 	let payload = Payload::from_single_entry(
		// 		known_payloads::ETF_SIGNATURE, 
		// 		Vec::new()
		// 	);
		// 	let commitment = Commitment { 
		// 		payload, 
		// 		block_number, 
		// 		validator_set_id,
		// 	};
		// 	if sig.verify(&Message::new(b"", &commitment.encode()), &rk) {
		// 		let bounded_sig = 
		// 			BoundedVec::<u8, ConstU32<48>>::try_from(raw_signature)
		// 				.map_err(|_| Error::<T>::InvalidSignature)?; // shouldn't ever happen?
		// 		let pulses = <Pulses<T>>::get();


		// 		// if valid, then interpolate the sig
				
		// 		// within the 'signature group', but not a double signature...
		// 		// let sig = interpolate_threshold_bls::<TinyBLS377>(vec![
		// 		//     (<TinyBLS377 as EngineBLS>::Scalar::from(1u8), sig_1.0),
		// 		//     (<TinyBLS377 as EngineBLS>::Scalar::from(2u8), sig_2.0),
		// 		//     (<TinyBLS377 as EngineBLS>::Scalar::from(3u8), sig_3.0),
		// 		// ]);


		// 		// note: the pulses list cannot be empty (set on genesis)
		// 		let mut last_pulse = Pulse::default();
		// 		if !pulses.is_empty() {
		// 			last_pulse = pulses[pulses.len() - 1].clone();
		// 		}
				
		// 		let pulse = Pulse::build_next(
		// 			bounded_sig, 
		// 			block_number, 
		// 			last_pulse
		// 		);
		// 		let mut pulses = <Pulses<T>>::get();
		// 		pulses.try_push(pulse.clone())
		// 			.map_err(|_| Error::<T>::PulseOverflow)?;
		// 		<Pulses<T>>::put(pulses);
		// 		return Ok(());
		// 	} else {
		// 		return Err(Error::<T>::InvalidSignature);
		// 	}
		// }

		// DOESNT WORK: only works w/ std... but that doesn't work here...
		// we could use an offchain worker potentially? but idk if I like that idea

		// let mut pub_keys_in_sig_grp: Vec<PublicKeyInSignatureGroup<TinyBLS377>> = Vec::new();

		// let pubkeys = <pallet_etf::Pallet<T>>::commitments();
		// let mut aggregated_public_key = PublicKey::<TinyBLS377>(<TinyBLS377 as EngineBLS>::PublicKeyGroup::zero());
		// pubkeys.iter().for_each(|pk| {
		// 	let sig_bytes: &[u8] = &pk.encode()[0..48];
		// 	let pub_bytes: &[u8] = &pk.encode()[48..144];
		// 	let pk_pub = <TinyBLS377 as EngineBLS>::PublicKeyGroup::deserialize_compressed(pub_bytes).unwrap();
		// 	let pk_sig = <TinyBLS377 as EngineBLS>::SignatureGroup::deserialize_compressed(sig_bytes).unwrap();
		// 	pub_keys_in_sig_grp.push(PublicKeyInSignatureGroup::<TinyBLS377>(pk_sig));
		// 	aggregated_public_key.0 += pk_pub;
		// });

		// let mut verifier_aggregator = SignatureAggregatorAssumingPoP::<TinyBLS377>::new(message);
		// let signature = Signature::<TinyBLS377>::from_bytes(&signature_bytes).unwrap();
		// verifier_aggregator.add_signature(&signature);
		// //aggregate public keys in signature group
		// verifier_aggregator.add_publickey(&aggregated_public_key);
		// pub_keys_in_sig_grp.iter().for_each(|pk| {verifier_aggregator.add_auxiliary_public_key(pk);});
		
		// if verifier_aggregator.verify_using_aggregated_auxiliary_public_keys::<Sha256>() {
		// 	let bounded_sig = 
		// 		BoundedVec::<u8, ConstU32<144>>::try_from(signature_bytes)
		// 			.map_err(|_| Error::<T>::InvalidSignature)?; // shouldn't ever happen?
		// 	let pulses = <Pulses<T>>::get();


		// 	// 	// if valid, then interpolate the sig?
				
		// 	// 	// within the 'signature group', but not a double signature...
		// 	// 	// let sig = interpolate_threshold_bls::<TinyBLS377>(vec![
		// 	// 	//     (<TinyBLS377 as EngineBLS>::Scalar::from(1u8), sig_1.0),
		// 	// 	//     (<TinyBLS377 as EngineBLS>::Scalar::from(2u8), sig_2.0),
		// 	// 	//     (<TinyBLS377 as EngineBLS>::Scalar::from(3u8), sig_3.0),
		// 	// 	// ]);


		// 		// note: the pulses list cannot be empty (set on genesis)
		// 		let mut last_pulse = Pulse::default();
		// 		if !pulses.is_empty() {
		// 			last_pulse = pulses[pulses.len() - 1].clone();
		// 		}
				
		// 		let pulse = Pulse::build_next(
		// 			bounded_sig, 
		// 			block_number, 
		// 			last_pulse
		// 		);
		// 		let mut pulses = <Pulses<T>>::get();
		// 		pulses.try_push(pulse.clone())
		// 			.map_err(|_| Error::<T>::PulseOverflow)?;
		// 		<Pulses<T>>::put(pulses);
		// 		return Ok(());
		// }

		// Err(Error::<T>::SignatureNotDeserializable)
	}
}
