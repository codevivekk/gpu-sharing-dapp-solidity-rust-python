'use client'

import JobRegistryABI from "../abi/JobRegistryABI.json";
import { useEffect, useMemo, useState } from "react";
import { useAppKitAccount } from "@reown/appkit/react";
import { ethers } from "ethers";

export const useJobRegistry = () => {
  const [signer, setSigner] = useState();
  const { isConnected } = useAppKitAccount();
  const contractAddress = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS;

  const contract = useMemo(() => {
    if (!signer || !contractAddress) return null;
    return new ethers.Contract(contractAddress, JobRegistryABI.abi, signer);
  }, [signer, contractAddress]);

  useEffect(() => {
    const initSigner = async () => {
      if (!window.ethereum) return;
      await window.ethereum.request({ method: "eth_requestAccounts" });
      const provider = new ethers.BrowserProvider(window.ethereum);
      const signer = await provider.getSigner();
      setSigner(signer);
    };

    if (isConnected) initSigner();
  }, [isConnected]);

  const createNode = async (specs, memoryAvailable, gpuName) => {
    if (!contract) throw new Error("Contract not ready");
    console.log("Registering node with specs:", specs, memoryAvailable, gpuName);
    console.log("Contract address:", contractAddress);
    const tx = await contract.registerNode(specs, memoryAvailable, gpuName);
    return tx.wait();
  };

  const createJob = async ({
    jobId,
    datasetCID,
    containerCID,
    deadline,
    requiredSpecs,
    minMemory,
    value = "0",
  }) => {
    if (!contract) throw new Error("Contract not ready");
    const tx = await contract.createJob(
      jobId,
      datasetCID,
      containerCID,
      deadline,
      requiredSpecs,
      minMemory,
      { value: ethers.parseEther(value) }
    );
    return tx.wait();
  };

  const claimJob = async (jobId) => {
    if (!contract) throw new Error("Contract not ready");
    const tx = await contract.claimJob(jobId);
    return tx.wait();
  };

  const assignProvider = async (jobId, providerAddress) => {
    if (!contract) throw new Error("Contract not ready");
    const tx = await contract.assignProvider(jobId, providerAddress);
    return tx.wait();
  };

  const submitResult = async (jobId, resultHash) => {
    if (!contract) throw new Error("Contract not ready");
    const tx = await contract.submitResult(jobId, resultHash);
    return tx.wait();
  };

  const release = async (jobId) => {
    if (!contract) throw new Error("Contract not ready");
    const tx = await contract.release(jobId);
    return tx.wait();
  };

  const extendDeadline = async (jobId, newDeadline) => {
    if (!contract) throw new Error("Contract not ready");
    const tx = await contract.extendDeadline(jobId, newDeadline);
    return tx.wait();
  };

  const jobCount = async () => {
    if (!contract) throw new Error("Contract not ready");
    const count = await contract.jobCount();
    return Number(count); 
  };

  const getAllJobs = async () => {
    if (!contract) throw new Error("Contract not ready");
    return contract.getAllJobs();
  };

  const getMyJobs = async () => {
    if (!contract) throw new Error("Contract not ready");
    return contract.getMyJobs();
  };

  return {
    contract,
    signer,
    createNode,
    createJob,
    claimJob,
    assignProvider,
    submitResult,
    release,
    extendDeadline,
    jobCount,
    getAllJobs,
    getMyJobs,
  };
};
