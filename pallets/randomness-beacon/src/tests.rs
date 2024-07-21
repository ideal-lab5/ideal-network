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

use std::vec;
use codec::Encode;
use frame_support::{assert_ok, traits::OnInitialize};
use crate::{
    self as beacon,
    BlockNumberFor,
    mock::*, 
    Call, Config, 
    Error, Weight
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use sp_core::{bls377::Signature, Pair, ByteArray};
use sp_consensus_beefy_etf::{
	Commitment, ValidatorSetId, Payload, known_payloads,
};

use etf_crypto_primitives::utils::interpolate_threshold_bls;
use w3f_bls::{DoublePublicKey, DoubleSignature, EngineBLS, Message, TinyBLS377, SerializableToBytes};

fn init_block(block: u64) {
	System::set_block_number(block);
	Session::on_initialize(block);
}

fn calculate_signature(id: u8, serialized_resharing: &[u8], message: &[u8]) -> Signature {
    let kp = sp_core::bls::Pair::from_seed_slice(&[id;32]).unwrap();
    let etf_kp = kp.acss_recover(serialized_resharing, 1).unwrap();
    etf_kp.sign(message)
}

#[test]
fn test_genesis() {
    // for simplicity of simulating a beacon, we use a single validator model
    new_test_ext(vec![1]).execute_with(|| {
        let pulses = beacon::Pulses::<Test>::get();
        assert!(pulses.is_empty());
    });
}

#[test]
fn test_can_write_single_pulse() {
	new_test_ext(vec![1]).execute_with(|| {
        let pulses = beacon::Pulses::<Test>::get();
        assert_eq!(pulses.len(), 0);

        let round_pk_bytes: Vec<u8> = <pallet_etf::Pallet<Test>>::round_pubkey().to_vec();
		let rk = DoublePublicKey::<TinyBLS377>::deserialize_compressed(
				&round_pk_bytes[..]
		).unwrap();
        // now we write a new pulse...
        let resharing_bytes = &pallet_etf::Shares::<Test>::get()[0];

        let payload = Payload::from_single_entry(
            known_payloads::ETF_SIGNATURE, 
            Vec::new()
        );
        let validator_set_id = <pallet_beefy::Pallet<Test>>::validator_set_id();
        let block_number: BlockNumberFor<Test> = 1;
        let commitment = Commitment { 
            payload, 
            block_number, 
            validator_set_id,
        };

        let signature = calculate_signature(1, resharing_bytes, &commitment.encode());
        let sig_bytes: &[u8] = signature.as_ref();
        let sig = DoubleSignature::<TinyBLS377>::from_bytes(sig_bytes).unwrap();
        // a little sanity check
        assert!(sig.verify(&Message::new(b"", &commitment.encode()), &rk));
        
        assert_ok!(Beacon::write_pulse(
            RuntimeOrigin::none(), 
            sig_bytes.to_vec(),
            1,
        ));
        // step to next block
        init_block(1);

        let pulses = beacon::Pulses::<Test>::get();
        assert_eq!(pulses.len(), 1);
	});
}

#[test]
fn test_can_write_many_pulses() {
	new_test_ext(vec![1]).execute_with(|| {
        let pulses = beacon::Pulses::<Test>::get();
        assert_eq!(pulses.len(), 0);

        let round_pk_bytes: Vec<u8> = <pallet_etf::Pallet<Test>>::round_pubkey().to_vec();
		let rk = DoublePublicKey::<TinyBLS377>::deserialize_compressed(
				&round_pk_bytes[..]
		).unwrap();
        // now we write a new pulse...
        let resharing_bytes = &pallet_etf::Shares::<Test>::get()[0];

        let payload = Payload::from_single_entry(
            known_payloads::ETF_SIGNATURE, 
            Vec::new()
        );
        let validator_set_id = <pallet_beefy::Pallet<Test>>::validator_set_id();
        let block_number: BlockNumberFor<Test> = 1;
        let commitment = Commitment { 
            payload: payload.clone(), 
            block_number, 
            validator_set_id,
        };

        let signature = calculate_signature(1, resharing_bytes, &commitment.encode());
        let sig_bytes: &[u8] = signature.as_ref();
        let sig = DoubleSignature::<TinyBLS377>::from_bytes(sig_bytes).unwrap();
        // a little sanity check
        assert!(sig.verify(&Message::new(b"", &commitment.encode()), &rk));
        
        assert_ok!(Beacon::write_pulse(
            RuntimeOrigin::none(), 
            sig_bytes.to_vec(),
            1,
        ));
        // step to next block
        init_block(1);

        let pulses = beacon::Pulses::<Test>::get();
        assert_eq!(pulses.len(), 1);

        // write another valid pulse
        let next_block_number: BlockNumberFor<Test> = 2;
        let validator_set_id = <pallet_beefy::Pallet<Test>>::validator_set_id();
        let next_commitment = Commitment { 
            payload, 
            block_number: next_block_number, 
            validator_set_id,
        };

        let next_signature = calculate_signature(1, resharing_bytes, &next_commitment.encode());
        let next_sig_bytes: &[u8] = next_signature.as_ref();

        assert_ok!(Beacon::write_pulse(
            RuntimeOrigin::none(), 
            next_sig_bytes.to_vec(),
            2,
        ));
        // step to next block
        init_block(2);

        let pulses = beacon::Pulses::<Test>::get();
        assert_eq!(pulses.len(), 2);

	});
}