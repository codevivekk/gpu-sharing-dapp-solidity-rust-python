# Decentralized GPU Job Scheduler (DePIN)

A proof-of-concept Decentralized Physical Infrastructure Network (DePIN) that connects job submitters with GPU providers in a trustless way on U2U Network


## Problem Statement

Alice, a machine learning engineer, struggles to find affordable GPU time for her experiments, while Axel, a GPU owner, leaves powerful hardware idle without a secure way to earn from it.

 This project solves these issues by creating a decentralized GPU job scheduling platform, where jobs are assigned automatically to providers and payments are handled trustlessly via smart contracts.

### Soution

#### Job Submission UI and Node Provider UI (Next.js)
– A job submitter, such as a researcher, developer, or machine learning engineer, begins by accessing the Frontend UI to create a new job. In the submission form, they provide key details including a dataset link (for example, an IPFS CID or an external URL), the container image (Docker or WASM environment required to run the workload), the bounty (payment amount that will later be released to the provider), and a deadline (the maximum time allowed for the job to complete). Once submitted, the frontend sends the job details to the Scheduler API, while also recording the job information on-chain through the smart contract, ensuring that the bounty is locked in escrow until successful completion.

- A provider who wants to contribute GPU power and registers their node. During registration, they provide details such as GPU specifications, memory, and their wallet address, which the system stores in the scheduler’s database.

#### Scheduler (Rust) 
– Once the Scheduler (Rust backend) receives a job, it stores the job details in its database (e.g., jobs.json). The scheduler then checks for available provider nodes from the node registry (nodes.json) and uses a selection strategy such as round-robin or resource matching to pick an idle provider node.

#### Node Agent (Python) 
– The Node Agent runs locally on the provider’s machine and acts as the bridge between the scheduler and the GPU hardware. Once started, the agent continuously polls the scheduler’s API at fixed intervals to check whether any jobs have been assigned to its node. If a job is found, the agent downloads the dataset (for example, from an IPFS link), pulls the specified container image (Docker or WASM), and executes the workload directly on the provider’s GPU. After execution, the agent generates a result hash or stores the output to IPFS, then submits the result and logs back to the scheduler. This allows the scheduler to verify completion, mark the job as successful, and trigger the smart contract to release the bounty to the provider’s wallet address.

#### Smart Contracts (Solidity) 
– The smart contract acts as the trustless payment layer of the system. When a job submitter creates a new job through the frontend, the bounty amount they specify is locked into the contract as escrow. The contract securely holds these funds until the job is completed. Once the provider’s Node Agent executes the job and submits the result, the scheduler verifies the completion and calls the contract to record the result and release the funds. The contract ensures that only the provider assigned to the job can submit results, checks that the deadline has not passed, and prevents duplicate submissions. If everything is valid, the bounty is transferred from escrow directly to the provider’s wallet address. This guarantees fair, automated, and tamper-proof payments without requiring trust between job submitters and providers.


### Example 
Alice’s Challenge:
Alice is a machine learning engineer working on a deep learning project. She needs to train a large model but lacks access to affordable GPU resources. Searching for cloud services proves costly and centralized, making her project inefficient.

Submitting a Job:
Alice accesses the platform’s Job Submission UI built in Next.js. She fills out the form with her dataset link (stored on IPFS), selects a Docker container for her workload, sets a bounty for the provider, and specifies a deadline. Once submitted, the job details are sent to the Scheduler API and recorded on-chain in a smart contract, locking her bounty in escrow until the job is successfully completed.

Axel Steps In:
Axel is a GPU owner with an idle workstation. He registers his node through the Node Provider UI, providing details like GPU type, memory, and his wallet address. The scheduler stores Axel’s node in its registry.

Scheduler Assignment:
The Rust-based Scheduler receives Alice’s job and looks for available providers. Using a selection strategy like round-robin or resource matching, it assigns the job to Axel’s idle node.

Node Agent Execution:
Axel’s Python Node Agent constantly polls the scheduler. Once it detects the job assignment, it downloads Alice’s dataset from IPFS, pulls the specified Docker container, and executes the workload on Axel’s GPU. After the job finishes, the agent generates a result hash, stores the output on IPFS, and submits the results back to the scheduler.

Trustless Payment:
The scheduler verifies the job completion and calls the smart contract. The contract ensures that the results are valid, the deadline is met, and only Axel can claim the bounty. Once verified, the escrowed funds are released directly to Axel’s wallet, guaranteeing a fair, automated, and tamper-proof payment.

Outcome:
Alice gets her compute-heavy task completed efficiently, and Axel earns money from his idle GPU without any trust issues. The decentralized system benefits both parties, creating an efficient, fair, and secure GPU marketplace.


## Project Setup

### Next JS


## Getting Started

```bash
1) git clone https://github.com/codevivekk/gpu-sharing-dapp-solidity-rust-python.git
2) cd ./frontend
3) npm install
4) npm run dev

```

Open [http://localhost:3001](http://localhost:3001) with your browser to see the result.


Setup env file as per as .env.example


### Solidity


## Getting Started
Setup env file as per as .env.example

```bash
1) cd ./contract
2) npm install
3) npx hardhat compile
4) npx hardhat run scripts/deploy.js --network u2uTestnet

```

### Rust

## Getting Started
Setup env file as per as .env.example

```bash
1) cd ./scheduler
2) cargo build
3) cargo watch -x run
```


### Python

## Getting Started
Setup env file as per as .env.example

```bash
1) cd ./node-agent
2) pip install -r requirements.txt
```
To ensure proper system integration and reward fulfillment, you must first register your node to obtain its unique Node ID. This ID is then essential for configuring the node-agent, which facilitates operational reporting and verification. Finally, execute the Python main file; successful completion of this script will validate the node's function within the network and trigger the release of your corresponding bounty.














