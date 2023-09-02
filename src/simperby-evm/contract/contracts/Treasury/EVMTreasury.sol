// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721Receiver.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "../Library/Verify.sol";
import "../Library/BytesLib.sol";
import "../Library/Utils.sol";
import "../Library/Strings.sol";
import "./interfaces/IEVMTreasury.sol";

contract EVMTreasury is ReentrancyGuard, IERC721Receiver, IEVMTreasury {
    using BytesLib for bytes;

    /// @notice The name of this contract
    string public constant name = "EVM SETTLEMENT CHAIN TREASURY V1";
    bytes public constant chainName = hex"6d797468657265756d"; // mythereum, for testing
    uint128 public contractSequence = 0;

    LightClient public lightClient;

    /* ========== CONSTRUCTOR ========== */
    constructor(bytes memory initialHeader) {
        Verify.BlockHeader memory _blockHeader = Verify.parseHeader(initialHeader);

        bytes32[] memory commitRoots = new bytes32[](1);
        commitRoots[0] = _blockHeader.commitMerkleRoot;

        lightClient = LightClient(_blockHeader.blockHeight, initialHeader, commitRoots);
    }

    /* ========== VIEW FUCNTIONS ========== */
    function viewCommitRoots() public view returns (bytes32[] memory) {
        return lightClient.commitRoots;
    }

    /* ========== TREASURY FUNCTIONS ========== */
    /**
     * @dev Functions to execute transactions.
     * @param transaction The transaction to be executed.
     * @param blockHeight The block height of the transaction.
     * @param merkleProof The merkle proof of the transaction.
     */
    function execute(
        bytes memory transaction,
        bytes memory executionData,
        uint64 blockHeight,
        bytes memory merkleProof
    ) public nonReentrant {
        bytes memory hashOfExecution = Strings.fromHex(
            string(transaction.slice(transaction.length - 68, 64))
        );
        require(
            bytes32(hashOfExecution) == keccak256(executionData),
            "EVMTreasury::execute: Invalid execution hash"
        );

//        Verify.verifyTransactionCommitment(
//            transaction,
//            lightClient.commitRoots,
//            merkleProof,
//            blockHeight,
//            lightClient.heightOffset
//        );

        (bytes memory _chainName, uint128 _contractSequence, uint32 msgType, uint offset) = Verify
            .parseExecutionData(executionData);

//        require(
//            keccak256(_chainName) == keccak256(chainName),
//            "EVMTreasury::execute: Invalid chain"
//        );
//        require(
//            _contractSequence == contractSequence,
//            "EVMTreasury::execute: Invalid contract sequence"
//        );

        if (msgType == 1) {
            FungibleTokenTransfer memory fungibleTokenTransfer = Verify.parseFTExecution(
                executionData,
                offset
            );

            if (fungibleTokenTransfer.tokenAddress == address(0)) {
                withdrawETH(fungibleTokenTransfer.receiverAddress, fungibleTokenTransfer.amount);
            } else {
                withdrawERC20(
                    fungibleTokenTransfer.tokenAddress,
                    fungibleTokenTransfer.receiverAddress,
                    fungibleTokenTransfer.amount
                );
            }
        } else if (msgType == 2) {
            NonFungibleTokenTransfer memory nonFungibleTokenTransfer = Verify.parseNFTExecution(
                executionData,
                offset
            );

            withdrawERC721(
                nonFungibleTokenTransfer.collectionAddress,
                nonFungibleTokenTransfer.receiverAddress,
                nonFungibleTokenTransfer.tokenId
            );
        } else {
            revert("Invalid transaction header");
        }

        contractSequence += 1;
    }

    function withdrawETH(address to, uint256 amount) internal {
        require(address(this).balance >= amount, "EVMTreasury::withdrawETH: Insufficient balance");
        emit TransferFungibleToken(address(0), amount, to, contractSequence);

        payable(to).transfer(amount);
    }

    function withdrawERC20(address token, address to, uint256 amount) internal {
        require(
            IERC20(token).balanceOf(address(this)) >= amount,
            "EVMTreasury::withdrawERC20: Insufficient balance"
        );
        IERC20(token).transfer(to, amount);

        emit TransferFungibleToken(token, amount, to, contractSequence);
    }

    function withdrawERC721(address token, address to, uint256 tokenId) internal {
        require(
            IERC721(token).ownerOf(tokenId) == address(this),
            "EVMTreasury::withdrawERC721: Insufficient balance"
        );
        IERC721(token).safeTransferFrom(address(this), to, tokenId);

        emit TransferNonFungibleToken(token, tokenId, to, contractSequence);
    }

    /* ========== LIGHTCLIENT FUNCTIONS ========== */
    /**
     * @dev Function to update light client.
     * @param header The header to be updated.
     * @param proof The finalization proof of the header.
     */
    function updateLightClient(bytes memory header, bytes calldata proof) public {
        Verify.BlockHeader memory _prevBlockHeader = Verify.parseHeader(lightClient.lastHeader);
        Verify.BlockHeader memory _blockHeader = Verify.parseHeader(header);
        Verify.BlockFinalizationProof memory _proof = Verify.parseProof(proof);
        bytes32 signingData = keccak256(abi.encodePacked(keccak256(header), _proof.round));

        Verify.verifyHeaderToHeader(lightClient.lastHeader, _prevBlockHeader, _blockHeader);
        Verify.verifyFinalizationProof(_blockHeader, signingData, _proof);

        lightClient.lastHeader = header;
        lightClient.commitRoots.push(_blockHeader.commitMerkleRoot);

        emit UpdateLightClient(_blockHeader.blockHeight, lightClient.lastHeader);
    }

    function onERC721Received(
        address,
        address,
        uint256,
        bytes calldata
    ) public pure override returns (bytes4) {
        return this.onERC721Received.selector;
    }
}
