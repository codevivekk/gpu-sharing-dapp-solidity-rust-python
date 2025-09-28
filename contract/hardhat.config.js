require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config();

module.exports = {
  solidity: "0.8.28",
  networks: {
    u2uTestnet: {
      url: process.env.U2U_RPC_URL,
      accounts: [process.env.PRIVATE_KEY],
      chainId: 2484,
    },
  },
};