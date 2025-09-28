"use client";

import { useState, useEffect } from "react";
import { useJobRegistry } from "../../../hooks/useJobRegistry";
import { ethers } from "ethers";

export default function JobList({}) {
  const { getAllJobs, contract } = useJobRegistry();
  const [allJobs, setAllJobs] = useState([]);
  console.log("All Jobs:", allJobs);

  useEffect(() => {
    const fetchJobs = async () => {
      try {
        const jobs = await getAllJobs();
        setAllJobs(jobs);
        console.log("Jobs from contract:", jobs);
      } catch (err) {
        console.error("Error fetching jobs:", err);
      }
    };

    contract && fetchJobs();
  }, [contract]);

  return (
    <div className="mx-5  mt-10">
      <h2 className="text-2xl font-bold mb-6">📋 Job List</h2>

      <div className="overflow-x-auto rounded-lg shadow border">
        <table className="w-full border-collapse text-sm">
          <thead className="bg-gray-100 text-left text-gray-700">
            <tr>
              <th className="p-3 border-b">Job Id</th>
              <th className="p-3 border-b">Owner</th>
              <th className="p-3 border-b">Deadline</th>
              <th className="p-3 border-b">Container</th>
              <th className="p-3 border-b">Dataset</th>
              <th className="p-3 border-b">Bounty</th>
              <th className="p-3 border-b">GPU Required</th>
              <th className="p-3 border-b">Memory Required</th>
              <th className="p-3 border-b">Result Hash</th>
              <th className="p-3 border-b">Provider</th>
              <th className="p-3 border-b">completed</th>
            </tr>
          </thead>
          <tbody>
            {allJobs.map((job) => {
              const valueInWei = ethers.formatEther(job[2]); // BigInt
              return (
                <tr
                  key={job.jobId + Math.random().toString()}
                  className={`${
                    false
                      ? "bg-green-50 border-l-4 border-green-400"
                      : "hover:bg-gray-50"
                  }`}
                >
                  {/* jobId */}
                  <td className="p-3 border-b font-mono">{job[0]}</td>
                  {/* owner */}
                  <td className="p-3 border-b font-mono">{job[1]}</td> 
                  {/* deadline  */}
                  <td className="p-3 border-b">{job[5]}</td>
                  {/* container */}
                  <td className="p-3 border-b">
                    {job[4]}
                  </td>
                  {/* Dataset */}
                  <td className="p-3 border-b">{job[3]}</td>
                  {/* Bounty */}
                  <td className="p-3 border-b">{valueInWei}</td>
                    {/* GUp Required */}
                  <td className="p-3 border-b">{job[8]}</td>
                   {/* Memory Required */}
                  <td className="p-3 border-b">{job[9]}</td>
                    {/* Result Hash */}
                  <td className="p-3 border-b">{job[7]}</td>
                      {/* Provuder */}
                  <td className="p-3 border-b">{job[6]}</td>
                  {/* completed */}
                  <td className="p-3 border-b">
                    <span className={`px-2 py-1 rounded text-xs font-medium `}>
                      {JSON.stringify(job[10])}
                    </span>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}
