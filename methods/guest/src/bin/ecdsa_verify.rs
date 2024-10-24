// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Read;

use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    EncodedPoint,
};
use risc0_zkvm::guest::env;

fn main() {
    // Decode the verifying key, message, and signature from the inputs.

    // let (encoded_verifying_key, message, signature): (EncodedPoint, Vec<u8>, Signature) =
    //     env::read();

    // let encoded_verifying_key: EncodedPoint = {
    //     let bytes: Vec<u8> = env::read();
    //     EncodedPoint::from_bytes(bytes).unwrap()
    // };
    // let message: Vec<u8> = env::read();
    // let signature: Signature = {
    //     let bytes: Vec<u8> = env::read();
    //     Signature::from_slice(&bytes).unwrap()
    // };

    let mut input_bytes: Vec<u8> = vec![];
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    let input_bytes = <Bytes>::abi_decode(&input_bytes, true).unwrap();

    let first: Vec<u8> = input_bytes[0..33].to_vec();
    let encoded_verifying_key: EncodedPoint = EncodedPoint::from_bytes(&first).unwrap();
    let verifying_key = VerifyingKey::from_encoded_point(&encoded_verifying_key).unwrap();

    let second: Vec<u8> = input_bytes[33..33 + 64].to_vec();
    let signature: Signature = Signature::from_slice(&second).unwrap();

    let message: Vec<u8> = input_bytes[33+64..].to_vec();

    // Verify the signature, panicking if verification fails.
    verifying_key
        .verify(&message, &signature)
        .expect("ECDSA signature verification failed");

    // Commit to the journal the verifying key and message that was signed.
    // env::commit(&(encoded_verifying_key, message));
    let mut output_bytes: Vec<u8> = vec![];
    output_bytes.extend_from_slice(&first);
    output_bytes.extend_from_slice(&message);
    let output = Bytes::copy_from_slice(&output_bytes);
    env::commit_slice(output.abi_encode().as_slice());
}
