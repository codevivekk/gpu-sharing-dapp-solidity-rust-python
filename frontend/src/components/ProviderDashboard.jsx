"use client";
// src/components/ProviderDashboard.jsx
import { useEffect, useState } from "react";
import axios from "axios";
import NodeRegistration from "./NodeRegistration";
// import JobCard from "./JobCard";

export default function ProviderDashboard() {
  const [node, setNode] = useState(null);
  const [jobs, setJobs] = useState([]);

  const fetchJobs = async () => {
    if (!node) return;
    try {
      const res = await axios.get(`http://localhost:4000/nodes/${node.nodeId}/jobs`);
      setJobs(res.data);
    } catch (err) {
      console.error(err);
    }
  };

  useEffect(() => {
    const interval = setInterval(fetchJobs, 10000); // poll every 10s
    return () => clearInterval(interval);
  }, [node]);

  const handleJobCompleted = (jobId) => {
    setJobs(prev => prev.map(j => j.id === jobId ? { ...j, status: "completed" } : j));
  };

  return (
    <div>
      {!node ? (
        <NodeRegistration onRegistered={setNode} />
      ) : (
        <div>
          <h2>Welcome, Node: {node.nodeId}</h2>
          <h3>GPU: {node.gpuName} ({node.memory}GB)</h3>
          <h3>Assigned Jobs:</h3>
          {jobs.length === 0 ? <p>No jobs assigned yet.</p> : jobs.map(job =>
            <JobCard key={job.id} job={job} nodeId={node.nodeId} onJobCompleted={handleJobCompleted} />
          )}
        </div>
      )}
    </div>
  );
}
