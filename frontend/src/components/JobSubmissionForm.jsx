"use client";
import React, { useState } from "react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useAppKitAccount } from "@reown/appkit/react";
import { useJobRegistry } from "../../hooks/useJobRegistry";
import axios from "axios";

export default function JobSubmissionForm() {
  const { createJob } = useJobRegistry();
  const { isConnected, address } = useAppKitAccount();

  const [form, setForm] = useState({
    datasetCID: "",
    containerCID: "",
    bounty: "",
    deadline: "",
    requiredSpecs: "",
    minMemory: "",
  });

  const [errors, setErrors] = useState({});
  const [loading, setLoading] = useState(false);
  const [notification, setNotification] = useState(null);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setForm((prev) => ({ ...prev, [name]: value }));
    setErrors((prev) => ({ ...prev, [name]: "" }));
  };

  const validate = () => {
    const newErrors = {};
    if (!form.datasetCID) newErrors.datasetCID = "Dataset CID is required";
    if (!form.containerCID)
      newErrors.containerCID = "Container CID is required";
    if (!form.bounty || parseFloat(form.bounty) <= 0)
      newErrors.bounty = "Bounty must be greater than 0";
    if (!form.deadline) newErrors.deadline = "Deadline is required";
    if (!form.requiredSpecs)
      newErrors.requiredSpecs = "Required specs are required";
    if (!form.minMemory || parseInt(form.minMemory) <= 0)
      newErrors.minMemory = "Minimum memory must be > 0";
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  async function postJob(jobData) {
    try {
      console.log("Posting job data:", jobData);
      const response = await axios.post(`${process.env.NEXT_PUBLIC_BASE_URL}/jobs`, jobData, {
        headers: {
          "Content-Type": "application/json",
        },
      });

      console.log("Job created:", response.data.job);
      return response.data.job;
    } catch (error) {
      console.error(
        "Error posting job:",
        error.response ? error.response.data : error.message
      );
    }
  }

  function generateJobId() {
    return "job-" + Math.random().toString(36).substring(2, 10); // e.g. node-x7k3a1b2
  }

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (!validate()) return;

    setLoading(true);
    setNotification(null);
    const jobId = generateJobId();

    try {
      const timestamp = BigInt(
        Math.floor(new Date(form.deadline).getTime() / 1000)
      );
      const job = {
        owner: address,
        jobId,
        bounty : form.bounty,
        deadline : form.deadline,
        dataset : form.datasetCID,
        containerCID: form.containerCID,
        status: "pending",
        requiredSpecs : form.requiredSpecs,
        minMemory : parseInt(form.minMemory),
        completed: false,
      };
      await postJob(job);
      const response = await createJob({
        jobId,
        datasetCID: form.datasetCID,
        containerCID: form.containerCID,
        deadline: timestamp,
        requiredSpecs: form.requiredSpecs,
        minMemory: parseInt(form.minMemory),
        value: form.bounty.toString(),
      });
console.log("Job creation transaction:", response);
      setNotification({
        type: "success",
        message: "Job submitted successfully!",
      });
      setForm({
        datasetCID: "",
        containerCID: "",
        bounty: "",
        deadline: "",
        requiredSpecs: "",
        minMemory: "",
        minReliabilityScore: "",
      });
    } catch (err) {
      console.error(err);
      setNotification({ type: "error", message: "Failed to submit job." });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="max-w-lg mx-auto mt-10">
      <Card className="shadow-xl rounded-2xl">
        <CardContent className="p-6">
          <h2 className="text-xl font-bold mb-4">Submit New Job</h2>
          <form onSubmit={handleSubmit} className="space-y-4">
            {[
              { label: "IPFS Dataset CID Link", name: "datasetCID", type: "text" },
              { label: "Docker Image name", name: "containerCID", type: "text" },
              { label: "Bounty Amount (U2U)", name: "bounty", type: "number" },
              { label: "Deadline", name: "deadline", type: "date" },
              { label: "Required Specs", name: "requiredSpecs", type: "text" },
              {
                label: "Minimum Memory (GB)",
                name: "minMemory",
                type: "number",
              },
            ].map((field) => (
              <div key={field.name}>
                <Label>{field.label}</Label>
                <Input
                  type={field.type}
                  name={field.name}
                  value={form[field.name]}
                  onChange={handleChange}
                  placeholder={field.label}
                  required
                />
                {errors[field.name] && (
                  <p className="text-red-500 text-sm mt-1">
                    {errors[field.name]}
                  </p>
                )}
              </div>
            ))}

            <Button
              type="submit"
              disabled={loading || !isConnected}
              className="w-full"
            >
              {loading ? "Submitting..." : "Submit Job"}
            </Button>
          </form>

          {notification && (
            <div
              className={`mt-4 p-2 rounded text-white ${
                notification.type === "success" ? "bg-green-600" : "bg-red-600"
              }`}
            >
              {notification.message}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
