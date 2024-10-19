// Copyright 2024 RISC Zero, Inc.
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
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";
import {console2} from "forge-std/console2.sol";
import {Test, console} from "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {Keccak256} from "../contracts/Keccak256.sol";
import {Elf} from "./Elf.sol"; // auto-generated contract after running `cargo build`.

contract Keccak256Test is RiscZeroCheats, Test {
    Keccak256 public keccak256_;

    function setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        keccak256_ = new Keccak256(verifier);
        assertEq(keccak256_.get(), bytes32(0));
    }

    function test_set_keccak256_hash_output() public {
        bytes32 input = bytes32(0);
        (bytes memory journal, bytes memory seal) = prove(Elf.KECCAK256_PATH, abi.encode(input));

        keccak256_.set(abi.decode(journal, (bytes32)), seal);
        assertEq(keccak256_.get(), bytes32(0x4220f7b47dc9e1f91e2d7c117a12e9158ce7a78185c805d21338759838f6f55d));
    }
}
