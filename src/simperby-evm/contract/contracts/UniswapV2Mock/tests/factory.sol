// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;
import "../UniswapV2Factory.sol";

contract factory is UniswapV2Factory{

    constructor() UniswapV2Factory(msg.sender){

    }

}