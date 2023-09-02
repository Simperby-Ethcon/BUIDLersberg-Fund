import { network, ethers, web3 } from "hardhat";
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers";

const E18n = 10n ** 18n;
const E18n_1M = 1_000_000n * E18n;

async function main() {
  const signers = await ethers.getSigners();

  const owner = signers[0];

  const name = "TestERC20";
  const name_721 = "TestERC721";

  const symbol = "TST";
  const symbol_721 = "TST721";

  const initialAccount = owner.address;
  const initialBalance = E18n_1M;

  console.log("Deploying contracts with the account:", owner.address);

  const ERC20Mock = await ethers.getContractFactory("ERC20Mock");
  const erc20mock = await ERC20Mock.deploy(
    name,
    symbol,
    initialAccount,
    initialBalance
  );

  const ERC721Mock = await ethers.getContractFactory("ERC721Mock");
  const erc721mock = await ERC721Mock.deploy(name_721, symbol_721);

  await erc20mock.deployed();
  await erc721mock.deployed();

  console.log(`ERC20Mock deployed at ${erc20mock.address} successfully`);
  console.log(`ERC721Mock deployed at ${erc721mock.address} successfully`);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
