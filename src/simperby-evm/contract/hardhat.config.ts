const path = require("path");
const envPath = path.join(__dirname, "./.env");
require("dotenv").config({ path: envPath });

import "@nomicfoundation/hardhat-foundry";
require("hardhat-deploy");
require("hardhat-contract-sizer");
require("hardhat-gas-reporter");
require("@nomiclabs/hardhat-waffle");
require("@nomiclabs/hardhat-truffle5");
require("@nomiclabs/hardhat-web3");
require("@nomiclabs/hardhat-etherscan");
require("@openzeppelin/hardhat-upgrades");
require("hardhat-spdx-license-identifier");

let mnemonic: string | undefined = process.env.MNEMONIC;
if (!mnemonic) {
  mnemonic = "test test test test test test test test test test test junk";
}

let infuraKey: string | undefined = process.env.INFURA_PROJECT_ID;
if (!infuraKey) {
  infuraKey = "9aa3d95b3bc440fa88ea12eaa4456161";
}

let coinmarketcap_api_key: string | undefined =
  process.env.COINMARKETCAP_API_KEY;

/**
 * @type import('hardhat/config').HardhatUserConfig
 */
module.exports = {
  defaultNetwork: "hardhat",
  abiExporter: {
    path: "./abi",
    clear: true,
    flat: true,
  },
  networks: {
    hardhat: {
      accounts: {
        mnemonic: mnemonic,
      },
    },
    // arbitrum: {
    //   url: process.env.ARBITRUM_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.ARBITRUM_MNEMONIC_PHRASE,
    //   },
    //   chainId: 42161,
    //   gas: "auto",
    //   gasPrice: 500000000, // 0.5 Gwei
    //   gasMultiplier: 1.2,
    // },
    // aurora: {
    //   url: process.env.AURORA_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.AURORA_MNEMONIC_PHRASE,
    //   },
    //   chainId: 1313161554,
    //   gas: "auto",
    //   gasPrice: 3500000000, // 3.5 Gwei
    //   gasMultiplier: 1.2,
    // },
    // avalanche: {
    //   url: process.env.AVALANCHE_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.AVALANCHE_MNEMONIC_PHRASE,
    //   },
    //   chainId: 43114,
    //   gas: "auto",
    //   gasPrice: 225000000000, // 225 Gwei
    //   gasMultiplier: 1.2,
    // },
    // boba: {
    //   url: process.env.BOBA_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.BOBA_MNEMONIC_PHRASE,
    //   },
    //   chainId: 288,
    //   gas: "auto",
    //   gasPrice: 12500000000, // 12.5 Gwei
    //   gasMultiplier: 1.2,
    // },
    // bsc: {
    //   url: process.env.BSC_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.BSC_MNEMONIC_PHRASE,
    //   },
    //   chainId: 56,
    //   gas: "auto",
    //   gasPrice: 6000000000, // 6 Gwei
    //   gasMultiplier: 1.2,
    // },
    ethereum: {
      url: `${process.env.ETHEREUM_NETWORK_ENDPOINT}`,
      accounts: {
        mnemonic: mnemonic,
      },
      chainId: 1,
      gas: "auto",
      gasPrice: 50000000000, // 50 Gwei
      gasMultiplier: 1.2,
    },
    goerli: {
      url: `https://goerli.infura.io/v3/${infuraKey}`,
      accounts: {
        mnemonic: mnemonic,
      },
      // accounts: [
      //   "YOUR PRIVATE KEY HERE",
      // ],
      chainId: 5,
      gas: "auto",
      gasPrice: "auto",
      gasMultiplier: 1.2,
    },
    // evmos: {
    //   url: process.env.EVMOS_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.EVMOS_MNEMONIC_PHRASE,
    //   },
    //   chainId: 9001,
    //   gas: "auto",
    //   gasPrice: 10000000000, // 10 Gwei
    //   gasMultiplier: 1.2,
    // },
    // fantom: {
    //   url: process.env.FANTOM_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.FANTOM_MNEMONIC_PHRASE,
    //   },
    //   chainId: 250,
    //   gas: "auto",
    //   gasPrice: 750000000000, // 750 Gwei
    //   gasMultiplier: 1.2,
    // },
    // fuse: {
    // 	url: process.env.FUSE_NETWORK_ENDPOINT,
    // 	accounts: {
    // 		mnemonic: process.env.FUSE_MNEMONIC_PHRASE
    // 	},
    // 	chainId: 122,
    // 	gas: "auto",
    // 	gasPrice: 5000000000, // 5 Gwei
    // 	gasMultiplier: 1.2
    // },
    // harmony: {
    //   url: process.env.HARMONY_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.HARMONY_MNEMONIC_PHRASE,
    //   },
    //   chainId: 1666600000,
    //   gas: "auto",
    //   gasPrice: 50000000000, // 50 Gwei
    //   gasMultiplier: 1.2,
    // },
    // moonbeam: {
    //   url: process.env.MOONBEAM_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.MOONBEAM_MNEMONIC_PHRASE,
    //   },
    //   chainId: 1284,
    //   gas: "auto",
    //   gasPrice: 150000000000, // 150 Gwei
    //   gasMultiplier: 1.2,
    // },
    // moonriver: {
    //   url: process.env.MOONRIVER_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.MOONRIVER_MNEMONIC_PHRASE,
    //   },
    //   chainId: 1285,
    //   gas: "auto",
    //   gasPrice: 3000000000, // 3 Gwei
    //   gasMultiplier: 1.2,
    // },
    // optimism: {
    //   url: process.env.OPTIMISM_NETWORK_ENDPOINT,
    //   accounts: {
    //     mnemonic: process.env.OPTIMISM_MNEMONIC_PHRASE,
    //   },
    //   chainId: 10,
    //   gas: "auto",
    //   gasPrice: 1000000000, // 1 Gwei
    //   gasMultiplier: 1.2,
    // },
    // polygon: {
    //   url: `${process.env.POLYGON_NETWORK_ENDPOINT}`,
    //   accounts: {
    //     mnemonic: process.env.POLYGON_MNEMONIC_PHRASE,
    //   },
    //   chainId: 137,
    //   gas: "auto",
    //   gasPrice: 75000000000, // 75 Gwei
    //   gasMultiplier: 1.2,
    // },
    // polygon_mumbai: {
    //   url: `${process.env.POLYGON_MUMBAI_NETWORK_ENDPOINT}`,
    //   accounts: {
    //     mnemonic: process.env.ROPSTEN_HARDHAT_PHRASE,
    //   },
    //   chainId: 80001,
    //   gas: "auto",
    //   gasPrice: 4000000000, // 4 Gwei
    //   gasMultiplier: 1.2,
    // },
    // zksync: {
    // 	url: process.env.ZKSYNC_NETWORK_ENDPOINT,
    // 	accounts: {
    // 		mnemonic: process.env.ZKSYNC_MNEMONIC_PHRASE
    // 	},
    // 	chainId: 123456,
    // 	gas: "auto",
    // 	gasPrice: 5000000000, // 5 Gwei
    // 	gasMultiplier: 1.2
    // },
  },
  solidity: {
    compilers: [
      {
        version: "0.8.4",
        settings: {
          optimizer: {
            enabled: true,
            runs: 100000,
          },
        },
      },
      {
        version: "0.8.6",
        settings: {
          optimizer: {
            enabled: true,
            runs: 100000,
          },
        },
      },
      {
        version: "0.8.10",
        settings: {
          optimizer: {
            enabled: true,
            runs: 100000,
          },
        },
      },
      {
        version: "0.8.13",
        settings: {
          optimizer: {
            enabled: true,
            runs: 100000,
          },
        },
      },
      {
        version: "0.8.15",
        settings: {
          optimizer: {
            enabled: true,
            runs: 100000,
          },
        },
      },
      {
        version: "0.8.16",
        settings: {
          optimizer: {
            enabled: true,
            runs: 100000,
          },
        },
      },
      {
        version: "0.8.17",
        settings: {
          optimizer: {
            enabled: true,
            runs: 100000,
          },
        },
      },
    ],
  },
  paths: {
    sources: "./contracts",
    tests: "./test/hardhat",
    cache: "./cache",
    artifacts: "./artifacts",
  },
  mocha: {
    timeout: 50000000,
  },
  etherscan: {
    apiKey: process.env.ETHERSCAN_API_KEY, // ETH Mainnet
  },
  gasReporter: {
    enabled: true,
    outputFile: "gas-report.txt",
    noColors: true,
    currency: "USD",
    coinmarketcap: coinmarketcap_api_key,
  },
  contractSizer: {
    alphaSort: true,
    runOnCompile: true,
    disambiguatePaths: false,
  },
};
