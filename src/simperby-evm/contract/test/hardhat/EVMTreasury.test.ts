import { time, loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { anyValue } from "@nomicfoundation/hardhat-chai-matchers/withArgs";
import { expect } from "chai";
// import { ethers, web3, waffle } from "hardhat";
import { Bytes } from "ethers";
import {
  initialHeader,
  nextHeader,
  fp,
  tx,
  merkleProof,
  execution,
} from "../../scripts/misc/constants";
import { ethers } from 'ethers';

const E18n = 10n ** 18n;
const E9n = 10n ** 9n;
const E6n = 10n ** 6n;
const E6_1M = 1_000_000n * E6n;
const E18_1M = 1_000_000n * E18n;
const E18_500K = 500_000n * E18n;

const contract_name: string = "EVM SETTLEMENT CHAIN TREASURY V1";

type UnPromisify<T> = T extends Promise<infer U> ? U : T;

// @dev: This test is not working properly since we need to link it with simperby.
// Simperby block header, transaction and other types should be updated to do the following test.

describe("EVMTreasury", function () {
  async function buildFixture() {
    const accounts = await ethers.getSigners();

    const [alice, bob, charlie] = accounts;

    const Treasury = await ethers.getContractFactory("EVMTreasury");
    const treasury = await Treasury.deploy(initialHeader);

    const ERC20Mock = await ethers.getContractFactory("ERC20Mock");
    const erc20 = await ERC20Mock.deploy(
      "TestERC20", // name
      "TST", // symbol
      alice.address, // intial account
      E18_1M // initial balance
    );

    const ERC721Mock = await ethers.getContractFactory("ERC721Mock");
    const erc721 = await ERC721Mock.deploy(
      "TestERC721", // name
      "TST721" // symbol
    );

    return { treasury, erc20, erc721, accounts, alice, bob, charlie };
  }
  let fixture: UnPromisify<ReturnType<typeof buildFixture>>;
  beforeEach(async function () {
    fixture = await loadFixture(buildFixture);
  });

  describe("Deployment", function () {
    it("Should set the right contract name", async function () {
      const { treasury } = fixture;

      expect(await treasury.name()).to.equal(contract_name);
    });
  });

  describe("Update light client - success", function () {
    it("Success case", async function () {
      const { treasury, erc20, alice, bob, charlie } = fixture;

      await expect(treasury.updateLightClient(nextHeader, fp)).to.emit(
        treasury,
        "UpdateLightClient"
      );

      expect((await treasury.lightClient()).lastHeader).to.equal(nextHeader);
    });
  });

  describe("Execution - success", function () {
    this.beforeEach(async function () {
      const { treasury, erc20, erc721, alice } = fixture;

      await erc20.connect(alice).transfer(treasury.address, E18_500K);

      expect(await erc20.balanceOf(treasury.address)).to.equal(E18_500K);
    });

    it("Transfer ERC20", async function () {
      const { treasury, erc20, alice } = fixture;

      await expect(treasury.updateLightClient(nextHeader, fp)).to.emit(
        treasury,
        "UpdateLightClient"
      );

      await expect(treasury.execute(tx, execution, 1, merkleProof)).to.emit(
        treasury,
        "TransferFungibleToken"
      );

      const afterBalance = await erc20.balanceOf(alice.address);

      // Transfer 100 tokens from treasury to alice
      expect(afterBalance).to.equal(500000000000000000000100n);
    });
  });
});
