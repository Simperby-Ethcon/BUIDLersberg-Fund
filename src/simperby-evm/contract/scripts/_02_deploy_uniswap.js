const hre = require("hardhat");

async function main() {
    const adminAdd = "0x3765DFeDf234Bf24f9df82715503B864236ddb48";
    const signers = await ethers.getSigners();
    // console.log(deployer);

    // lpTokenContract = new ethers.Contract(lpTokenAddress, lpTokenAbi, signer[0]);

    const GetInit = await ethers.getContractFactory("CalHash");
    getInit = await GetInit.deploy();
    initHash = await getInit.connect(signers[0]).getInitHash();

    const LpToken = await ethers.getContractFactory("UniswapV2ERC20");
    const lpToken = await LpToken.deploy();
    await lpToken.deployed();
    console.log("lpToken address: ", lpToken.address);

    const WETH = await ethers.getContractFactory("WETH9");
    const weth = await WETH.deploy();
    await weth.deployed();
    console.log("weth address: ", weth.address);

    const UniswapV2Factory = await ethers.getContractFactory(
        "UniswapV2Factory"
    );
    const uniswapV2Factory = await UniswapV2Factory.deploy(adminAdd);
    await uniswapV2Factory.deployed();
    console.log("uniswapV2Factory address: ", uniswapV2Factory.address);

    const UniswapV2Router = await ethers.getContractFactory(
        "UniswapV2Router02"
    );
    let uniswapV2Router = await UniswapV2Router.deploy(
        uniswapV2Factory.address,
        weth.address,
        signers[1].address
    );
    await uniswapV2Router.deployed();
    console.log(uniswapV2Router.address, "uniswapV2Router.address");

//     const TUniswapV2Factory = await ethers.getContractFactory(
//         "UniswapV2Factory"
//     );
//     const TuniswapV2Factory = await TUniswapV2Factory.deploy(adminAdd);
//     await TuniswapV2Factory.deployed();
//     console.log("uniswapV2Factory address: ", TuniswapV2Factory.address);

//     const TUniswapV2Router = await ethers.getContractFactory(
//         "UniswapV2Router02"
//     );
//     let TuniswapV2Router = await TUniswapV2Router.deploy(
//         TuniswapV2Factory.address,
//         weth.address
//     );
//     await TuniswapV2Router.deployed();
//     console.log(TuniswapV2Router.address, "uniswapV2Router.address");

    // const UniswapV2Pair = await ethers.getContractFactory("UniswapV2Pair");
    // const uniswapV2Pair = await UniswapV2Pair.deploy();
    // await uniswapV2Pair.deployed();
    // console.log("uniswapV2Pair address: ", uniswapV2Pair.address);

    // const TokenG = await ethers.getContractFactory("TokenG");
    // const tokenG = await TokenG.deploy();
    // await tokenG.deployed();
    // console.log("Token G address: ", tokenG.address);

    // const TokenB = await ethers.getContractFactory("TokenB");
    // const tokenB = await TokenB.deploy();
    // await tokenB.deployed();
    // console.log("Token B address: ", tokenB.address);

    // const TokenC = await ethers.getContractFactory("TokenC");
    // const tokenC = await TokenC.deploy();
    // await tokenC.deployed();
    // console.log("Token C address: ", tokenC.address);

    // const TokenD = await ethers.getContractFactory("TokenD");
    // const tokenD = await TokenD.deploy();
    // await tokenD.deployed();
    // console.log("Token D address: ", tokenD.address);

    // const TokenE = await ethers.getContractFactory("TokenE");
    // const tokenE = await TokenE.deploy();
    // await tokenE.deployed();
    // console.log("Token E address: ", tokenE.address);

    // const TokenF = await ethers.getContractFactory("TokenF");
    // const tokenF = await TokenF.deploy();
    // await tokenF.deployed();
    // console.log("Token F address: ", tokenF.address);

    // const TaxableToken = await ethers.getContractFactory("TokenF");
    // const taxableToken = await TaxableToken.deploy();
    // await taxableToken.deployed();
    // console.log("Token F address: ", taxableToken.address);

    // console.log(`Verifying contract on Etherscan...`);

    // await run(`verify:verify`, {
    //   address: "0x2c65D08F3d37882DE2226466F798f2980686157B"
    // //   constructorArguments: [priceFeedAddress],
    // });
}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });

// weth address:  0xaAd08F309763D8Ccc7fED412118885513Ad92074
// uniswapV2Factory address:  0x273afCc2a77ac582D30655b252EDC1AD7524448f
// uniswapV2Pair address:  0xD0Fd5462AeDBc2A4e5DCD53024c6D4367907b3b1
// uniswapV2Router.address: 0x0CFFa65dE864A0B984a1Ef15A8A6d0D8f27f30cD

// Token A address:  0x61A04ED710442fF8B4feff9C61c0Fb53DD052DF6
// Token B address:  0x570feFbFc3802fcd26eE87E37acb0B43a06f9703
// Token C address:  0x47136B54E791164EBE67d7878a4da0fa6f30E42f
// Token D address:  0x172A5B177d824E2Ded7441D26D5a859E6652a268
// Token E address:  0x8A6858Aa59A3192b3345F5E38aB9cF72E293915e
// Token F address:  0xcF37CA4D13A1a59397BA96eC329e3f14a8379FcE
