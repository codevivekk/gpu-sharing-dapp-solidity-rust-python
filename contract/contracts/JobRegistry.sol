// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

contract JobRegistry {
    struct Job {
        string jobId;  
        address owner;
        uint256 bounty;
        string datasetCID; 
        string containerCID; 
        uint256 deadline;
        address provider;
        bytes32 resultHash;
        string requiredSpecs;
        uint256 minMemory;   
        bool completed;
    }

    struct Node {
        address wallet;   
        string specs;       
        uint256 memoryAvailable;  
        string gpuName;  
        bool active;      
    }

    uint256 public jobCount;

    mapping(address => Node) public providers;
    mapping(string => Job) public jobs;
    string[] public jobIds;

function createJob(
    string memory jobId,
    string memory datasetCID,
    string memory containerCID,
    uint256 deadline,
    string memory requiredSpecs,
    uint256 minMemory
) external payable {
    require(jobs[jobId].owner == address(0), "Job already exists");

    jobs[jobId] = Job({
        jobId: jobId,
        owner: msg.sender,
        bounty: msg.value,
        datasetCID: datasetCID,
        containerCID: containerCID,
        deadline: deadline,
        provider: address(0),
        resultHash: 0x0,
        completed: false,
        requiredSpecs: requiredSpecs,
        minMemory: minMemory
    });
    jobIds.push(jobId);
    jobCount++;
}

    function getJobIds() external view returns (string[] memory) {
        return jobIds;
    }

    function claimJob(string memory jobId) external {
        Job storage job = jobs[jobId];
        Node storage node = providers[msg.sender];

        require(job.provider == address(0), "Job already claimed");
        require(!job.completed, "Job already completed");
        require(job.owner != msg.sender, "You cannot claim your own job");
        require(node.active, "Node not registered");
        require(node.memoryAvailable >= job.minMemory, "Insufficient memory");

        job.provider = msg.sender;
    }

    function submitResult(string memory jobId, bytes32 resultHash) external {
        Job storage job = jobs[jobId];
        require(job.provider == msg.sender, "Only assigned provider can submit");
        require(!job.completed, "Job already completed");
        // require(block.timestamp <= job.deadline, "Deadline passed");

        job.resultHash = resultHash;
        job.completed = true;
    }

    function registerNode(
        string memory specs,
        uint256 memoryAvailable,
        string memory gpuName
    ) external {
        providers[msg.sender] = Node({
            wallet: msg.sender,
            specs: specs,
            gpuName: gpuName,
            memoryAvailable: memoryAvailable,
            active: true
        });
    }

    function release(string memory jobId) external {
        Job storage job = jobs[jobId];
        require(job.completed, "Job not completed yet");
        require(msg.sender == job.owner, "Only owner can release bounty");

        address payable provider = payable(job.provider);
        uint256 amount = job.bounty;
        job.bounty = 0;

        provider.transfer(amount);
    }

    function extendDeadline(string memory jobId, uint256 newDeadline) external {
        Job storage job = jobs[jobId];
        require(msg.sender == job.owner, "Only owner can extend deadline");
        require(!job.completed, "Cannot extend completed job");
        require(newDeadline > job.deadline, "New deadline must be later");

        job.deadline = newDeadline;
    }


    function assignProvider(string memory jobId, address provider) external {
        Job storage job = jobs[jobId];
        // require(msg.sender == job.owner, "Only owner can assign provider");
        // require(job.provider == address(0), "Provider already assigned");
        require(!job.completed, "Job already completed");

        job.provider = provider;
    }

    function getMyJobs() external view returns (Job[] memory) {
        uint256 count = 0;
        for (uint256 i = 0; i < jobIds.length; i++) {
            if (jobs[jobIds[i]].owner == msg.sender) {
                count++;
            }
        }

        Job[] memory result = new Job[](count);
        uint256 index = 0;
        for (uint256 i = 0; i < jobIds.length; i++) {
            if (jobs[jobIds[i]].owner == msg.sender) {
                result[index] = jobs[jobIds[i]];
                index++;
            }
        }
        return result;
    }

    function getAllJobs() external view returns (Job[] memory) {
        Job[] memory result = new Job[](jobIds.length);
        for (uint256 i = 0; i < jobIds.length; i++) {
            result[i] = jobs[jobIds[i]];
        }
        return result;
    }

    function getPendingJobs() external view returns (Job[] memory) {
        uint256 count = 0;
        for (uint256 i = 0; i < jobIds.length; i++) {
            if (!jobs[jobIds[i]].completed) {
                count++;
            }
        }

        Job[] memory result = new Job[](count);
        uint256 index = 0;
        for (uint256 i = 0; i < jobIds.length; i++) {
            if (!jobs[jobIds[i]].completed) {
                result[index] = jobs[jobIds[i]];
                index++;
            }
        }
        return result;
    }

}
