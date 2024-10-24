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
import {EcdsaVerify} from "../contracts/EcdsaVerify.sol";
import {Elf} from "./Elf.sol"; // auto-generated contract after running `cargo build`.

contract EcdsaVerifyTest is RiscZeroCheats, Test {
    EcdsaVerify public ecdsaVerify;

    function setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        ecdsaVerify = new EcdsaVerify(verifier);
    }

    function test_ecdsa_verify_function() public {
        bytes memory input = bytes(hex"0353085ba8399b9a4b186a9e749987d643a879354f6c295f944a0cc45fb6504374cbf87664b121b6374a200626e1a06dfa9ff16d9e0620491b912eff6ac210390b68a260e780ec70210ce3e5cf88ea958e78416174095a23ec64f8692717e3dde8546869732069732061206d65737361676520746861742077696c6c206265207369676e65642c20616e642076657269666965642077697468696e20746865207a6b564d");
        (bytes memory journal, bytes memory seal) = prove(Elf.ECDSA_VERIFY_PATH, abi.encode(input));

        bytes memory journal_ = abi.decode(journal, (bytes));
        ecdsaVerify.verify(journal_, seal);
        // assertEq(ecdsaVerify.get(), bytes32(0x4220f7b47dc9e1f91e2d7c117a12e9158ce7a78185c805d21338759838f6f55d));
    }
}
