// const { expect } = require("chai");
// const { ethers } = require("hardhat");
// const { hre } = require("hardhat");
// const helpers = require("@nomicfoundation/hardhat-network-helpers");
// const { Description } = require("@ethersproject/properties");

// const provider = new ethers.providers.JsonRpcProvider("http://127.0.0.1:8545/");

// const wethAddress = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
// const wethABI = [
//   {
//     constant: true,
//     inputs: [],
//     name: "name",
//     outputs: [{ name: "", type: "string" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { name: "guy", type: "address" },
//       { name: "wad", type: "uint256" },
//     ],
//     name: "approve",
//     outputs: [{ name: "", type: "bool" }],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "totalSupply",
//     outputs: [{ name: "", type: "uint256" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { name: "src", type: "address" },
//       { name: "dst", type: "address" },
//       { name: "wad", type: "uint256" },
//     ],
//     name: "transferFrom",
//     outputs: [{ name: "", type: "bool" }],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [{ name: "wad", type: "uint256" }],
//     name: "withdraw",
//     outputs: [],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "decimals",
//     outputs: [{ name: "", type: "uint8" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [{ name: "", type: "address" }],
//     name: "balanceOf",
//     outputs: [{ name: "", type: "uint256" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "symbol",
//     outputs: [{ name: "", type: "string" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { name: "dst", type: "address" },
//       { name: "wad", type: "uint256" },
//     ],
//     name: "transfer",
//     outputs: [{ name: "", type: "bool" }],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [],
//     name: "deposit",
//     outputs: [],
//     payable: true,
//     stateMutability: "payable",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [
//       { name: "", type: "address" },
//       { name: "", type: "address" },
//     ],
//     name: "allowance",
//     outputs: [{ name: "", type: "uint256" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   { payable: true, stateMutability: "payable", type: "fallback" },
//   {
//     anonymous: false,
//     inputs: [
//       { indexed: true, name: "src", type: "address" },
//       { indexed: true, name: "guy", type: "address" },
//       { indexed: false, name: "wad", type: "uint256" },
//     ],
//     name: "Approval",
//     type: "event",
//   },
//   {
//     anonymous: false,
//     inputs: [
//       { indexed: true, name: "src", type: "address" },
//       { indexed: true, name: "dst", type: "address" },
//       { indexed: false, name: "wad", type: "uint256" },
//     ],
//     name: "Transfer",
//     type: "event",
//   },
//   {
//     anonymous: false,
//     inputs: [
//       { indexed: true, name: "dst", type: "address" },
//       { indexed: false, name: "wad", type: "uint256" },
//     ],
//     name: "Deposit",
//     type: "event",
//   },
//   {
//     anonymous: false,
//     inputs: [
//       { indexed: true, name: "src", type: "address" },
//       { indexed: false, name: "wad", type: "uint256" },
//     ],
//     name: "Withdrawal",
//     type: "event",
//   },
// ];

// const daiAddress = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
// const daiABI = [
//   {
//     inputs: [{ internalType: "uint256", name: "chainId_", type: "uint256" }],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "constructor",
//   },
//   {
//     anonymous: false,
//     inputs: [
//       { indexed: true, internalType: "address", name: "src", type: "address" },
//       { indexed: true, internalType: "address", name: "guy", type: "address" },
//       { indexed: false, internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "Approval",
//     type: "event",
//   },
//   {
//     anonymous: true,
//     inputs: [
//       { indexed: true, internalType: "bytes4", name: "sig", type: "bytes4" },
//       { indexed: true, internalType: "address", name: "usr", type: "address" },
//       { indexed: true, internalType: "bytes32", name: "arg1", type: "bytes32" },
//       { indexed: true, internalType: "bytes32", name: "arg2", type: "bytes32" },
//       { indexed: false, internalType: "bytes", name: "data", type: "bytes" },
//     ],
//     name: "LogNote",
//     type: "event",
//   },
//   {
//     anonymous: false,
//     inputs: [
//       { indexed: true, internalType: "address", name: "src", type: "address" },
//       { indexed: true, internalType: "address", name: "dst", type: "address" },
//       { indexed: false, internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "Transfer",
//     type: "event",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "DOMAIN_SEPARATOR",
//     outputs: [{ internalType: "bytes32", name: "", type: "bytes32" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "PERMIT_TYPEHASH",
//     outputs: [{ internalType: "bytes32", name: "", type: "bytes32" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [
//       { internalType: "address", name: "", type: "address" },
//       { internalType: "address", name: "", type: "address" },
//     ],
//     name: "allowance",
//     outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { internalType: "address", name: "usr", type: "address" },
//       { internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "approve",
//     outputs: [{ internalType: "bool", name: "", type: "bool" }],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [{ internalType: "address", name: "", type: "address" }],
//     name: "balanceOf",
//     outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { internalType: "address", name: "usr", type: "address" },
//       { internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "burn",
//     outputs: [],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "decimals",
//     outputs: [{ internalType: "uint8", name: "", type: "uint8" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [{ internalType: "address", name: "guy", type: "address" }],
//     name: "deny",
//     outputs: [],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { internalType: "address", name: "usr", type: "address" },
//       { internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "mint",
//     outputs: [],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { internalType: "address", name: "src", type: "address" },
//       { internalType: "address", name: "dst", type: "address" },
//       { internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "move",
//     outputs: [],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "name",
//     outputs: [{ internalType: "string", name: "", type: "string" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [{ internalType: "address", name: "", type: "address" }],
//     name: "nonces",
//     outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { internalType: "address", name: "holder", type: "address" },
//       { internalType: "address", name: "spender", type: "address" },
//       { internalType: "uint256", name: "nonce", type: "uint256" },
//       { internalType: "uint256", name: "expiry", type: "uint256" },
//       { internalType: "bool", name: "allowed", type: "bool" },
//       { internalType: "uint8", name: "v", type: "uint8" },
//       { internalType: "bytes32", name: "r", type: "bytes32" },
//       { internalType: "bytes32", name: "s", type: "bytes32" },
//     ],
//     name: "permit",
//     outputs: [],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { internalType: "address", name: "usr", type: "address" },
//       { internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "pull",
//     outputs: [],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { internalType: "address", name: "usr", type: "address" },
//       { internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "push",
//     outputs: [],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [{ internalType: "address", name: "guy", type: "address" }],
//     name: "rely",
//     outputs: [],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "symbol",
//     outputs: [{ internalType: "string", name: "", type: "string" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "totalSupply",
//     outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { internalType: "address", name: "dst", type: "address" },
//       { internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "transfer",
//     outputs: [{ internalType: "bool", name: "", type: "bool" }],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: false,
//     inputs: [
//       { internalType: "address", name: "src", type: "address" },
//       { internalType: "address", name: "dst", type: "address" },
//       { internalType: "uint256", name: "wad", type: "uint256" },
//     ],
//     name: "transferFrom",
//     outputs: [{ internalType: "bool", name: "", type: "bool" }],
//     payable: false,
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [],
//     name: "version",
//     outputs: [{ internalType: "string", name: "", type: "string" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     constant: true,
//     inputs: [{ internalType: "address", name: "", type: "address" }],
//     name: "wards",
//     outputs: [{ internalType: "uint256", name: "", type: "uint256" }],
//     payable: false,
//     stateMutability: "view",
//     type: "function",
//   },
// ];

// const whale = "0x06920C9fC643De77B99cB7670A944AD31eaAA260";
// const sendAddress = "0xb90F789eD58e7cF3eFc0421Ed87bb79f53B0f984";
// // let daiAddress = '0x6B175474E89094C44Da98b954EedeAC495271d0F';

// let impersonatedSigner;
// let wethContract;
// let tokenA;
// let uniswapContract;

// let uniswapAddress = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
// let uniswapAbi = [
//   {
//     inputs: [
//       { internalType: "address", name: "_factory", type: "address" },
//       { internalType: "address", name: "_WETH", type: "address" },
//     ],
//     stateMutability: "nonpayable",
//     type: "constructor",
//   },
//   {
//     inputs: [],
//     name: "WETH",
//     outputs: [{ internalType: "address", name: "", type: "address" }],
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "address", name: "tokenA", type: "address" },
//       { internalType: "address", name: "tokenB", type: "address" },
//       { internalType: "uint256", name: "amountADesired", type: "uint256" },
//       { internalType: "uint256", name: "amountBDesired", type: "uint256" },
//       { internalType: "uint256", name: "amountAMin", type: "uint256" },
//       { internalType: "uint256", name: "amountBMin", type: "uint256" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "addLiquidity",
//     outputs: [
//       { internalType: "uint256", name: "amountA", type: "uint256" },
//       { internalType: "uint256", name: "amountB", type: "uint256" },
//       { internalType: "uint256", name: "liquidity", type: "uint256" },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "address", name: "token", type: "address" },
//       { internalType: "uint256", name: "amountTokenDesired", type: "uint256" },
//       { internalType: "uint256", name: "amountTokenMin", type: "uint256" },
//       { internalType: "uint256", name: "amountETHMin", type: "uint256" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "addLiquidityETH",
//     outputs: [
//       { internalType: "uint256", name: "amountToken", type: "uint256" },
//       { internalType: "uint256", name: "amountETH", type: "uint256" },
//       { internalType: "uint256", name: "liquidity", type: "uint256" },
//     ],
//     stateMutability: "payable",
//     type: "function",
//   },
//   {
//     inputs: [],
//     name: "factory",
//     outputs: [{ internalType: "address", name: "", type: "address" }],
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountOut", type: "uint256" },
//       { internalType: "uint256", name: "reserveIn", type: "uint256" },
//       { internalType: "uint256", name: "reserveOut", type: "uint256" },
//     ],
//     name: "getAmountIn",
//     outputs: [{ internalType: "uint256", name: "amountIn", type: "uint256" }],
//     stateMutability: "pure",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountIn", type: "uint256" },
//       { internalType: "uint256", name: "reserveIn", type: "uint256" },
//       { internalType: "uint256", name: "reserveOut", type: "uint256" },
//     ],
//     name: "getAmountOut",
//     outputs: [{ internalType: "uint256", name: "amountOut", type: "uint256" }],
//     stateMutability: "pure",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountOut", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//     ],
//     name: "getAmountsIn",
//     outputs: [
//       { internalType: "uint256[]", name: "amounts", type: "uint256[]" },
//     ],
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountIn", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//     ],
//     name: "getAmountsOut",
//     outputs: [
//       { internalType: "uint256[]", name: "amounts", type: "uint256[]" },
//     ],
//     stateMutability: "view",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountA", type: "uint256" },
//       { internalType: "uint256", name: "reserveA", type: "uint256" },
//       { internalType: "uint256", name: "reserveB", type: "uint256" },
//     ],
//     name: "quote",
//     outputs: [{ internalType: "uint256", name: "amountB", type: "uint256" }],
//     stateMutability: "pure",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "address", name: "tokenA", type: "address" },
//       { internalType: "address", name: "tokenB", type: "address" },
//       { internalType: "uint256", name: "liquidity", type: "uint256" },
//       { internalType: "uint256", name: "amountAMin", type: "uint256" },
//       { internalType: "uint256", name: "amountBMin", type: "uint256" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "removeLiquidity",
//     outputs: [
//       { internalType: "uint256", name: "amountA", type: "uint256" },
//       { internalType: "uint256", name: "amountB", type: "uint256" },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "address", name: "token", type: "address" },
//       { internalType: "uint256", name: "liquidity", type: "uint256" },
//       { internalType: "uint256", name: "amountTokenMin", type: "uint256" },
//       { internalType: "uint256", name: "amountETHMin", type: "uint256" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "removeLiquidityETH",
//     outputs: [
//       { internalType: "uint256", name: "amountToken", type: "uint256" },
//       { internalType: "uint256", name: "amountETH", type: "uint256" },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "address", name: "token", type: "address" },
//       { internalType: "uint256", name: "liquidity", type: "uint256" },
//       { internalType: "uint256", name: "amountTokenMin", type: "uint256" },
//       { internalType: "uint256", name: "amountETHMin", type: "uint256" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "removeLiquidityETHSupportingFeeOnTransferTokens",
//     outputs: [{ internalType: "uint256", name: "amountETH", type: "uint256" }],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "address", name: "token", type: "address" },
//       { internalType: "uint256", name: "liquidity", type: "uint256" },
//       { internalType: "uint256", name: "amountTokenMin", type: "uint256" },
//       { internalType: "uint256", name: "amountETHMin", type: "uint256" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//       { internalType: "bool", name: "approveMax", type: "bool" },
//       { internalType: "uint8", name: "v", type: "uint8" },
//       { internalType: "bytes32", name: "r", type: "bytes32" },
//       { internalType: "bytes32", name: "s", type: "bytes32" },
//     ],
//     name: "removeLiquidityETHWithPermit",
//     outputs: [
//       { internalType: "uint256", name: "amountToken", type: "uint256" },
//       { internalType: "uint256", name: "amountETH", type: "uint256" },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "address", name: "token", type: "address" },
//       { internalType: "uint256", name: "liquidity", type: "uint256" },
//       { internalType: "uint256", name: "amountTokenMin", type: "uint256" },
//       { internalType: "uint256", name: "amountETHMin", type: "uint256" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//       { internalType: "bool", name: "approveMax", type: "bool" },
//       { internalType: "uint8", name: "v", type: "uint8" },
//       { internalType: "bytes32", name: "r", type: "bytes32" },
//       { internalType: "bytes32", name: "s", type: "bytes32" },
//     ],
//     name: "removeLiquidityETHWithPermitSupportingFeeOnTransferTokens",
//     outputs: [{ internalType: "uint256", name: "amountETH", type: "uint256" }],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "address", name: "tokenA", type: "address" },
//       { internalType: "address", name: "tokenB", type: "address" },
//       { internalType: "uint256", name: "liquidity", type: "uint256" },
//       { internalType: "uint256", name: "amountAMin", type: "uint256" },
//       { internalType: "uint256", name: "amountBMin", type: "uint256" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//       { internalType: "bool", name: "approveMax", type: "bool" },
//       { internalType: "uint8", name: "v", type: "uint8" },
//       { internalType: "bytes32", name: "r", type: "bytes32" },
//       { internalType: "bytes32", name: "s", type: "bytes32" },
//     ],
//     name: "removeLiquidityWithPermit",
//     outputs: [
//       { internalType: "uint256", name: "amountA", type: "uint256" },
//       { internalType: "uint256", name: "amountB", type: "uint256" },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountOut", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "swapETHForExactTokens",
//     outputs: [
//       { internalType: "uint256[]", name: "amounts", type: "uint256[]" },
//     ],
//     stateMutability: "payable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountOutMin", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "swapExactETHForTokens",
//     outputs: [
//       { internalType: "uint256[]", name: "amounts", type: "uint256[]" },
//     ],
//     stateMutability: "payable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountOutMin", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "swapExactETHForTokensSupportingFeeOnTransferTokens",
//     outputs: [],
//     stateMutability: "payable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountIn", type: "uint256" },
//       { internalType: "uint256", name: "amountOutMin", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "swapExactTokensForETH",
//     outputs: [
//       { internalType: "uint256[]", name: "amounts", type: "uint256[]" },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountIn", type: "uint256" },
//       { internalType: "uint256", name: "amountOutMin", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "swapExactTokensForETHSupportingFeeOnTransferTokens",
//     outputs: [],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountIn", type: "uint256" },
//       { internalType: "uint256", name: "amountOutMin", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "swapExactTokensForTokens",
//     outputs: [
//       { internalType: "uint256[]", name: "amounts", type: "uint256[]" },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountIn", type: "uint256" },
//       { internalType: "uint256", name: "amountOutMin", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "swapExactTokensForTokensSupportingFeeOnTransferTokens",
//     outputs: [],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountOut", type: "uint256" },
//       { internalType: "uint256", name: "amountInMax", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "swapTokensForExactETH",
//     outputs: [
//       { internalType: "uint256[]", name: "amounts", type: "uint256[]" },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   {
//     inputs: [
//       { internalType: "uint256", name: "amountOut", type: "uint256" },
//       { internalType: "uint256", name: "amountInMax", type: "uint256" },
//       { internalType: "address[]", name: "path", type: "address[]" },
//       { internalType: "address", name: "to", type: "address" },
//       { internalType: "uint256", name: "deadline", type: "uint256" },
//     ],
//     name: "swapTokensForExactTokens",
//     outputs: [
//       { internalType: "uint256[]", name: "amounts", type: "uint256[]" },
//     ],
//     stateMutability: "nonpayable",
//     type: "function",
//   },
//   { stateMutability: "payable", type: "receive" },
// ];

// describe("", async () => {

//   beforeEach(async () => {
//     impersonatedSigner = await ethers.getImpersonatedSigner(whale);
//     wethContract = new ethers.Contract(
//       wethAddress,
//       wethABI,
//       impersonatedSigner
//     );
//     uniswapContract = new ethers.Contract(
//       uniswapAddress,
//       uniswapAbi,
//       impersonatedSigner
//     );
//     const TokenA = await ethers.getContractFactory("TokenA");
//     tokenA = await TokenA.connect(impersonatedSigner).deploy();
//     await tokenA.deployed();
//   });

//   it("", async () => {
//     // console.log(await wethContract.balanceOf(sendAddress));
//     // await wethContract.connect(impersonatedSigner).approve(sendAddress,100);
//     // await wethContract.connect(impersonatedSigner).transfer(sendAddress,100);
//     // console.log(await wethContract.balanceOf(sendAddress));
//     // console.log(await provider.getBalance(whale));
//     // let tx = await impersonatedSigner.sendTransaction({
//     //   to: '0xb90F789eD58e7cF3eFc0421Ed87bb79f53B0f984',
//     //   value: ethers.utils.parseEther("1") // 1 ether
//     // })
//     // tx.wait();
//     // console.log(await provider.getBalance(whale));

//     await wethContract.connect(impersonatedSigner).approve(uniswapAddress, 100);

//     await uniswapContract
//       .connect(impersonatedSigner)
//       .swapExactTokensForTokens(
//         "100",
//         1,
//         [wethAddress, daiAddress],
//         impersonatedSigner.address,
//         1664553782
//       );
//   });
// });
