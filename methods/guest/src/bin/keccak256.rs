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

use risc0_zkvm::guest::env;
use sha3::{Digest, Keccak256};

fn main() {
    // Read the input data for this application.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();

    // Decode and parse the input
    assert!(input_bytes.len() == 32, "invalid inputs");

    // Run the computation.
    // In this case, compute keccak hash

    let mut output = input_bytes;
    for _ in 0..64 {
        output = keccak_hash(&output);
    }

    // Commit the journal that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.
    env::commit_slice(output.as_slice());
}

fn keccak_hash(input: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let output = hasher.finalize();
    output.to_vec()
}