import { network, ethers, web3 } from "hardhat";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";
import { initialHeader } from "./misc/constants";

async function main() {
  const chain_name = "Ethereum";
  const accounts = await ethers.getSigners();

  const signers = await ethers.getSigners();

  const owner = signers[0];

  console.log("Deploying contracts with the account:", owner.address);

  const Treasury = await ethers.getContractFactory("EVMTreasury");
  const treasury = await Treasury.deploy(initialHeader);

  await treasury.deployed();

  console.log(
    `EVM Treasury of ${chain_name} deployed at ${treasury.address} successfully`
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
