// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {EVMTreasury} from "../../contracts/Treasury/EVMTreasury.sol";
import {ERC20Mock} from "../../contracts/ERC20Mock/ERC20Mock.sol";
import {ERC721Mock} from "../../contracts/ERC721Mock/ERC721Mock.sol";
import {console} from "../../lib/forge-std/src/console.sol";
import {stdStorage, StdStorage, Test, Vm} from "../../lib/forge-std/src/Test.sol";
import "./utils/TestParameters.sol";

abstract contract Parameters {
    bytes internal genesisHeader = TestParameters.genesisHeader;
    bytes internal nextHeader = TestParameters.nextHeader;
    bytes internal transaction = TestParameters.transaction;
    bytes internal execution = TestParameters.execution;
    bytes internal merkleProof = TestParameters.merkleProof;
    bytes internal fp = TestParameters.fp;
}

contract TreasurySetup is Test, Parameters {
    EVMTreasury internal treasury;
    ERC20Mock internal erc20;
    ERC721Mock internal erc721;
    address payable[] internal users;

    function setUp() public virtual {
        users = new address payable[](2);
        users[0] = payable(address(0x1));
        users[1] = payable(address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266));
        treasury = new EVMTreasury(genesisHeader);
        erc20 = new ERC20Mock("USD Tether", "USDT", address(treasury), 100e18);
        erc721 = new ERC721Mock("SIMPERBY", "SPB");
        erc721.mint(address(treasury), 0);
    }
}

contract EVMTreasuryTest is TreasurySetup {
    function setUp() public virtual override {
        super.setUp();
        console.log("EVMTreasuryTest.setUp");
    }

    function testCheckConstant() public {
        console.log("EVMTreasuryTest.testCheckConstant");

        assertEq(treasury.name(), "EVM SETTLEMENT CHAIN TREASURY V1");
        assertEq(treasury.chainName(), hex"6d797468657265756d");
        assertEq(treasury.contractSequence(), 0);
        assertEq(erc20.balanceOf(address(treasury)), 100e18);
    }

    function testCheckInitialHeaderDecoding() public {
        console.log("EVMTreasuryTest.testCheckInitialHeaderDecoding");

        (uint256 heightOffset, bytes memory header) = treasury.lightClient();

        assertEq(heightOffset, 0);
        assertEq(header, genesisHeader);
    }

    function testUpdateLightClient() public {
        console.log("EVMTreasuryTest.testUpdateLightClient");

        treasury.updateLightClient(nextHeader, fp);
        (uint256 heightOffset, bytes memory header) = treasury.lightClient();
        bytes32[] memory commitRoots = treasury.viewCommitRoots();

        assertEq(heightOffset, 0);
        assertEq(header, nextHeader);
        assertEq(commitRoots.length, 2);
    }

    function testERC20Execution() public {
        console.log("EVMTreasuryTest.testERC20Execution");

        treasury.updateLightClient(nextHeader, fp);

        uint256 balanceOfUser = erc20.balanceOf(users[1]);
        assertEq(balanceOfUser, 0);

        treasury.execute(transaction, execution, 1, merkleProof);

        balanceOfUser = erc20.balanceOf(users[1]);
        assertEq(balanceOfUser, 100);
    }

    function testFailERC20ExecutionWrongBlockHeight() public {
        console.log("EVMTreasuryTest.testERC20Execution");

        treasury.updateLightClient(nextHeader, fp);
        treasury.execute(transaction, execution, 4, merkleProof); // Wrong block height
    }
}
