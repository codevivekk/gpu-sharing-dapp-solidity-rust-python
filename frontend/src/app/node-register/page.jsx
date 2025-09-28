"use client";
import React, { useState, useEffect } from "react";
import axios from "axios";
import { useAppKitAccount } from "@reown/appkit/react";
import { useJobRegistry } from "../../../hooks/useJobRegistry";

export default function NodeRegistration() {
  const [gpuSpecs, setGpuSpecs] = useState("");
  const [gpuName, setGpuName] = useState("");
  const [memory, setMemory] = useState("");
  const [loading, setLoading] = useState(false);
  const [notification, setNotification] = useState(null);
  const [nodes, setNodes] = useState([]);
  const { address, isConnected } = useAppKitAccount();
  const { createNode } = useJobRegistry();

  useEffect(() => {
    const fetchNodes = async () => {
      try {
        const res = await axios.get(
          `${process.env.NEXT_PUBLIC_BASE_URL}/nodes`
        );
        setNodes(res.data);
      } catch (err) {
        console.error("Failed to fetch nodes", err);
      }
    };
    fetchNodes();
  }, []);

  function generateNodeId() {
    return "node-" + Math.random().toString(36).substring(2, 10);
  }

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setNotification(null);

    try {
      const nodeId = generateNodeId();

      const res = await axios.post(
        `${process.env.NEXT_PUBLIC_BASE_URL}/nodes/register`,
        {
          nodeId,
          gpuName,
          gpuSpecs,
          owner: address,
          memoryAvailable: parseInt(memory),
        }
      );

      await createNode(gpuSpecs, memory, gpuName);

      setNotification({
        type: "success",
        message: "Node registered successfully!",
      });
      setNodes((prev) => [...prev, res.data]);
      setGpuSpecs("");
      setGpuName("");
      setMemory("");
    } catch (err) {
      console.error(err);
      setNotification({ type: "error", message: "Failed to register node." });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="mt-10">
      <div className="bg-white shadow-lg rounded-2xl p-6 border max-w-3xl mx-auto ">
        <h2 className="text-xl font-bold mb-6">Node Registration</h2>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              GPU Name
            </label>
            <input
              type="text"
              value={gpuName}
              onChange={(e) => setGpuName(e.target.value)}
              placeholder="Your Custom GPU Name"
              className="w-full border rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              GPU Specs
            </label>
            <input
              type="text"
              value={gpuSpecs}
              onChange={(e) => setGpuSpecs(e.target.value)}
              placeholder="NVIDIA RTX 3090"
              className="w-full border rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Memory Specs
            </label>
            <input
              type="text"
              value={memory}
              onChange={(e) => setMemory(e.target.value)}
              placeholder="24GB RAM"
              className="w-full border rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Your Address
            </label>
            <input
              type="text"
              value={address}
              disabled={true}
              className="w-full border rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>

          <button
            type="submit"
            disabled={loading || !isConnected}
            className="w-full bg-blue-600 text-white py-2 px-4 rounded-lg hover:bg-blue-700 transition disabled:opacity-50"
          >
            {loading ? "Registering..." : "Register Node"}
          </button>
        </form>

        {notification && (
          <p
            className={`mt-4 text-sm font-medium ${
              notification.type === "success"
                ? "text-green-600"
                : "text-red-600"
            }`}
          >
            {notification.message}
          </p>
        )}
      </div>

      <AllNodes nodes={nodes} />
    </div>
  );
}

function AllNodes({ nodes }) {
  return (
    <div className="mt-10 mx-20">
      <h3 className="text-lg font-semibold mb-4">🖥️ Registered Nodes</h3>
      <div className="overflow-x-auto rounded-lg shadow border">
        <table className="w-full border-collapse text-sm">
          <thead className="bg-gray-100 text-left text-gray-700">
            <tr>
              <th className="p-3 border-b">Node ID</th>
              <th className="p-3 border-b">Owner</th>
              <th className="p-3 border-b">GPU Name</th>
              <th className="p-3 border-b">GPU Specs</th>
              <th className="p-3 border-b">Status</th>
            </tr>
          </thead>
          <tbody>
            {nodes.map((node) => {
              let statusClass = "bg-gray-100 text-gray-700";
              if (node.status === "idle") {
                statusClass = "bg-green-100 text-green-700";
              } else if (node.status === "busy") {
                statusClass = "bg-blue-100 text-blue-700";
              }
              return (
                <tr key={node.nodeId} className="hover:bg-gray-50">
                  <td className="p-3 border-b font-mono">{node.nodeId}</td>
                  <td className="p-3 border-b font-mono">{node.owner}</td>
                  <td className="p-3 border-b font-mono">{node.gpuName}</td>
                  <td className="p-3 border-b font-mono">{node.gpuSpecs}</td>
                  <td className="p-3 border-b">
                    <span
                      className={`px-2 py-1 rounded text-xs font-medium ${statusClass}`}
                    >
                      {node.status || "unknown"}
                    </span>
                  </td>
                </tr>
              );
            })}

            {nodes.length === 0 && (
              <tr>
                <td colSpan="5" className="p-4 text-center text-gray-500">
                  No nodes registered yet.
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
