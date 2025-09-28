const { ethers } = require("hardhat");

async function main() {
    const taskManager = await ethers.getContractFactory("JobRegistry");
    const contract = await taskManager.deploy();
    await contract.waitForDeployment();
    
    const contractAddress = await contract.getAddress();
    console.log(`Contract deployed at: ${contractAddress}`);
}

main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});