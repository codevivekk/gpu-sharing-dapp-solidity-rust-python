"use client";
import { useState } from "react";
import axios from "axios";
import { Input } from "./ui/input";
import { Button } from "./ui/button";
import { Card, CardContent } from "./ui/card";

export default function NodeRegistration() {
  const [nodeId, setNodeId] = useState("");
  const [gpuName, setGpuName] = useState("");
  const [memory, setMemory] = useState("");

  const handleRegister = async () => {
    try {
      await axios.post("http://localhost:3000/nodes/register", {
        nodeId,
        gpuName,
        memory,
        status: "idle",
      });
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div className="node-registration max-w-lg mx-auto mt-10">
      <Card className="shadow-xl rounded-2xl">
        <CardContent className="p-6">
          <h2 className="text-xl font-bold mb-4">Register New Node</h2>
          <form className="space-y-4">
            <Input
              placeholder="Node ID"
              value={nodeId}
              onChange={(e) => setNodeId(e.target.value)}
            />
            <Input
              placeholder="GPU Name"
              value={gpuName}
              onChange={(e) => setGpuName(e.target.value)}
            />
            <Input
              placeholder="Memory (GB)"
              value={memory}
              onChange={(e) => setMemory(e.target.value)}
            />
            <Button type="button" onClick={handleRegister}>
              Register
            </Button>
          </form>
        </CardContent>
      </Card>
    </div>
  );
}
