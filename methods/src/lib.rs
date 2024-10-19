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

//! Generated crate containing the image ID and ELF binary of the build guest.
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[cfg(test)]
mod tests {
    use risc0_zkvm::{default_executor, ExecutorEnv};

    #[test]
    fn proves_keccak_hash_output() {
        let input: Vec<u8> = vec![0; 32];

        let env = ExecutorEnv::builder()
            .write_slice(&input)
            .build()
            .unwrap();

        // NOTE: Use the executor to run tests without proving.
        let session_info = default_executor().execute(env, super::KECCAK256_ELF).unwrap();
        let output = &session_info.journal.bytes;
        assert_eq!(*output, vec![66, 32, 247, 180, 125, 201, 225, 249, 30, 45, 124, 17, 122, 18, 233, 21, 140, 231, 167, 129, 133, 200, 5, 210, 19, 56, 117, 152, 56, 246, 245, 93]);
    }
}
