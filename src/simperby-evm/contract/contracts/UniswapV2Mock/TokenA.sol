// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/draft-ERC20Permit.sol";

contract TokenA is ERC20, ERC20Permit {
    constructor() ERC20("MyToken", "MTK") ERC20Permit("MyToken") {
        _mint(msg.sender, 10000000 * 10 ** decimals());
    }
}


    // constructor() ERC20("TokenA", "TKNA") {
    //     _mint(msg.sender, 10000000 * 10 ** decimals());
    // }