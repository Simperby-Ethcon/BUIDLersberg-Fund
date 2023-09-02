// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./BytesLib.sol";
import "./Utils.sol";
import "./Strings.sol";
import "../Treasury/interfaces/IEVMTreasury.sol";

/**
 * @dev Functions to verify signature & decode bytes
 */
library Verify {
    using BytesLib for bytes;

    /**
     * @dev Bytes length for decoding data.
     * @notice Refer to https://github.com/bincode-org/bincode/blob/trunk/docs/spec.md for details.
     * @notice We need to remove first 1 bytes prefix from {pkLength}.
     */
    uint constant sigLength = 65;
    uint constant pkLength = 65;
    uint constant hashLength = 32;
    uint constant addressLength = 20;
    uint constant uint128Length = 16;
    uint constant strUint64Length = 8;
    uint constant enumLength = 4;

    struct TypedSignature {
        bytes signature;
        bytes signer;
    }

    struct BlockFinalizationProof {
        uint64 round;
        TypedSignature[] blockFinalizationSignatures;
    }

    struct ValidatorSet {
        bytes validator;
        uint64 votingPower;
    }

    struct BlockHeader {
        bytes author;
        BlockFinalizationProof prevBlockFinalizationProof;
        bytes32 previousHash;
        uint64 blockHeight;
        int64 timestamp;
        bytes32 commitMerkleRoot;
        ValidatorSet[] validators;
        bytes version;
    }

    /* ========== VERIFY FUNCTIONS ========== */
    /**
     * @dev Verifies new header is valid.
     * @param prevHeader Bytes of a previous header (Current lastHeader).
     * @param _prevBlockHeader Decoded BlockHeader of a previous header.
     * @param _blockHeader Decoded BlockHeader of a new BlockHeader.
     */
    function verifyHeaderToHeader(
        bytes memory prevHeader,
        BlockHeader memory _prevBlockHeader,
        BlockHeader memory _blockHeader
    ) internal pure {
        require(
            _prevBlockHeader.blockHeight + 1 == _blockHeader.blockHeight,
            "Verify::verifyHeaderToHeader: Invalid block height"
        );
        require(
            _blockHeader.previousHash == keccak256(prevHeader),
            "Verify::verifyHeaderToHeader: Invalid previous hash"
        );
        require(
            _blockHeader.timestamp >= _prevBlockHeader.timestamp,
            "Verify::verifyHeaderToHeader: Invalid block timestamp"
        );

        for (uint i = 0; i < _prevBlockHeader.validators.length; i++) {
            if (
                keccak256(_prevBlockHeader.validators[i].validator) ==
                keccak256(_blockHeader.author)
            ) {
                break;
            } else {
                if (i == _prevBlockHeader.validators.length - 1) {
                    revert("Verify::verifyHeaderToHeader: Invalid block author");
                }
            }
        }

        bytes32 finalizationSignData = keccak256(
            abi.encodePacked(
                _blockHeader.previousHash,
                _blockHeader.prevBlockFinalizationProof.round
            )
        );
        verifyFinalizationProof(
            _prevBlockHeader,
            finalizationSignData,
            _blockHeader.prevBlockFinalizationProof
        );
    }

    /**
     * @dev Verifies finalization proof with TypedSignature.
     * @param header Decoded header.
     * @param finalizationSignTarget Signing data.
     * @param finalizationProof BlockFinalizationProof.
     */
    function verifyFinalizationProof(
        BlockHeader memory header,
        bytes32 finalizationSignTarget,
        BlockFinalizationProof memory finalizationProof
    ) internal pure {
        uint256 _totalVotingPower;
        uint256 _votedVotingPower;
        for (uint i = 0; i < header.validators.length; i++) {
            _totalVotingPower += header.validators[i].votingPower;
        }
        TypedSignature[] memory signatures = finalizationProof.blockFinalizationSignatures;
        for (uint j = 0; j < signatures.length; j++) {
            (bytes32 r, bytes32 s, uint8 v) = splitSignature(signatures[j].signature);
            if (
                Utils.pkToAddress(signatures[j].signer) ==
                ecrecover(finalizationSignTarget, v, r, s)
            ) {
                require(
                    keccak256(signatures[j].signer) == keccak256(header.validators[j].validator)
                );
                _votedVotingPower += header.validators[j].votingPower;
            }
        }

        require(
            _votedVotingPower * 3 > _totalVotingPower * 2,
            "Verify::verifyFinalizationProof: Not enough voting power"
        );
    }

    function verifyTransactionCommitment(
        bytes memory transaction,
        bytes32[] memory commitRoots,
        bytes memory merkleProof,
        uint64 blockHeight,
        uint64 heightOffset
    ) internal pure {
        require(
            blockHeight >= heightOffset && blockHeight < heightOffset + commitRoots.length,
            "Verify::verifyTransactionCommitment: Invalid block height"
        );

        bytes32 root = commitRoots[blockHeight - heightOffset];
        bytes32 calculatedRoot = keccak256(transaction);

        uint offset = 0;
        uint64 lenOfProof = Utils.reverse64(merkleProof.toUint64(offset));
        offset += strUint64Length;

        for (uint i = 0; i < lenOfProof; i++) {
            uint32 enumOrder = Utils.reverse32(merkleProof.toUint32(offset));
            offset += enumLength;

            if (enumOrder == 0) {
                // Left child
                bytes32 leftPairHash = merkleProof.toBytes32(offset);
                calculatedRoot = keccak256(abi.encodePacked(leftPairHash, calculatedRoot));
                offset += hashLength;
            } else if (enumOrder == 1) {
                // Right child
                bytes32 rightPairHash = merkleProof.toBytes32(offset);
                calculatedRoot = keccak256(abi.encodePacked(calculatedRoot, rightPairHash));
                offset += hashLength;
            } else {
                revert("Invalid enum order in merkle proof");
            }
        }

        require(
            root == calculatedRoot,
            "Verify::verifyTransactionCommitment: Merkle proof verification fail"
        );
    }

    /* ========== DECODER ========== */
    function splitSignature(
        bytes memory signature
    ) internal pure returns (bytes32 r, bytes32 s, uint8 v) {
        require(signature.length == 65, "invalid signature length");

        assembly {
            // first 32 bytes, after the length prefix
            r := mload(add(signature, 32))
            // second 32 bytes
            s := mload(add(signature, 64))
            // final byte (first byte of the next 32 bytes)
            v := byte(0, mload(add(signature, 96)))
        }
    }

    function parseProof(bytes memory input) internal pure returns (BlockFinalizationProof memory) {
        uint64 round = Utils.reverse64(input.toUint64(0));
        uint offset = strUint64Length;

        uint64 len = Utils.reverse64(input.toUint64(offset));
        offset += strUint64Length;

        require(
            len == (input.length - 16) / 130 && (input.length - 16) % 130 == 0,
            "Verify::parseProof: Invalid proof length"
        );

        TypedSignature[] memory fp = new TypedSignature[](len);

        for (uint256 i = 0; i < len; i++) {
            fp[i] = TypedSignature(
                input.slice(offset, sigLength),
                input.slice(offset + sigLength + 1, pkLength - 1)
            );
            offset += (sigLength + pkLength);
        }

        return BlockFinalizationProof(round, fp);
    }

    function parseHeader(
        bytes memory hexEncodedData
    ) internal pure returns (BlockHeader memory blockHeader) {
        uint offset = 0;

        blockHeader.author = hexEncodedData.slice(offset + 1, pkLength - 1);
        offset += pkLength;

        {
            uint64 round = Utils.reverse64(hexEncodedData.toUint64(offset));
            offset += strUint64Length;
            uint64 len = Utils.reverse64(hexEncodedData.toUint64(offset));
            offset += strUint64Length;
            TypedSignature[] memory signatures = new TypedSignature[](len);

            bytes memory _sig;
            bytes memory _signer;

            if (len != 0) {
                for (uint i = 0; i < len; i++) {
                    _sig = hexEncodedData.slice(offset, sigLength);
                    offset += sigLength;
                    _signer = hexEncodedData.slice(offset + 1, pkLength - 1);
                    offset += pkLength;

                    signatures[i] = TypedSignature(_sig, _signer);
                }
                blockHeader.prevBlockFinalizationProof = BlockFinalizationProof(round, signatures);
            }
        }

        blockHeader.previousHash = hexEncodedData.toBytes32(offset);
        offset += hashLength;

        blockHeader.blockHeight = Utils.reverse64(hexEncodedData.toUint64(offset));
        offset += strUint64Length;

        blockHeader.timestamp = int64(Utils.reverse64(hexEncodedData.toUint64(offset)));
        offset += strUint64Length;

        blockHeader.commitMerkleRoot = hexEncodedData.toBytes32(offset);
        offset += hashLength;

        // Skip repository root (32 bytes)
        offset += hashLength;

        {
            uint64 validatorsLen = Utils.reverse64(hexEncodedData.toUint64(offset));
            offset += strUint64Length;
            blockHeader.validators = new ValidatorSet[](validatorsLen);

            bytes memory _validator;
            uint64 _votingPower;

            for (uint i = 0; i < validatorsLen; i++) {
                _validator = hexEncodedData.slice(offset + 1, pkLength - 1);
                offset += pkLength;
                _votingPower = Utils.reverse64(hexEncodedData.toUint64(offset));
                offset += strUint64Length;

                blockHeader.validators[i] = ValidatorSet(_validator, _votingPower);
            }
        }

        // length of version is always 5, so ignore it.
        blockHeader.version = hexEncodedData.slice(offset + strUint64Length, 5);
    }

    function parseExecutionData(
        bytes memory executionData
    )
        internal
        pure
        returns (bytes memory chainName, uint128 contractSequence, uint32 msgType, uint offset)
    {
        uint64 lenOfChain = Utils.reverse64(executionData.toUint64(0));
        offset += strUint64Length;

        chainName = executionData.slice(offset, lenOfChain);
        offset += lenOfChain;

        contractSequence = Utils.reverse128(executionData.toUint128(offset));
        offset += uint128Length;

        msgType = Utils.reverse32(executionData.toUint32(offset));
        offset += enumLength;
    }

    function parseFTExecution(
        bytes memory executionData,
        uint offset
    ) internal pure returns (IEVMTreasury.FungibleTokenTransfer memory fungibleTokenTransfer) {
        fungibleTokenTransfer.tokenAddress = executionData.toAddress(offset);
        offset += addressLength;

        uint64 lenOfHex = Utils.reverse64(executionData.toUint64(offset));
        offset += strUint64Length;

        fungibleTokenTransfer.amount = uint128(
            Strings.stringToUint(string(executionData.slice(offset, lenOfHex)))
        );
        offset += lenOfHex;

        fungibleTokenTransfer.receiverAddress = executionData.toAddress(offset);
    }

    function parseNFTExecution(
        bytes memory executionData,
        uint offset
    )
        internal
        pure
        returns (IEVMTreasury.NonFungibleTokenTransfer memory nonFungibleTokenTransfer)
    {
        nonFungibleTokenTransfer.collectionAddress = executionData.toAddress(offset);
        offset += addressLength;

        nonFungibleTokenTransfer.tokenId = Utils.reverse128(executionData.toUint128(offset));
        offset += uint128Length;

        nonFungibleTokenTransfer.receiverAddress = executionData.toAddress(offset);
    }
}
