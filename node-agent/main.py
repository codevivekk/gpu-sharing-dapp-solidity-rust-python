import requests
import time
import hashlib
import os
from execute_job import execute_job
from config import config

w3 = config["w3"]
contract = config["contract"]
account = config["account"]
wallet_address = config["wallet_address"]

SCHEDULER_URL = "http://localhost:3000"
POLL_INTERVAL = 10  # seconds
MY_GPU = "Apple M3 GPU"
NODE_ID = "598b6167-112b-44d7-874f-bcd79c319b4e"
MY_ADDRESS = "0x1F1f090EEAF77Faae3D626fF7847682B7f66Fc8f"  

def assign_provider(job_id, wallet_address):
    payload = {
        "job_id": job_id,
        "address": wallet_address
    }
    API_URL = f"{SCHEDULER_URL}/nodes/assign-provider"
    print(API_URL)
    print(payload)

    
    try:
        response = requests.post(API_URL, json=payload, timeout=10)
        response.raise_for_status()
        data = response.json()
        print(f"[+] Job assigned successfully: {data}")
        return data
    except requests.exceptions.HTTPError as errh:
        print(f"[!] HTTP error: {errh} - {response.text}")
    except requests.exceptions.ConnectionError as errc:
        print(f"[!] Connection error: {errc}")
    except requests.exceptions.Timeout as errt:
        print(f"[!] Timeout error: {errt}")
    except requests.exceptions.RequestException as err:
        print(f"[!] Something went wrong: {err}")


def get_assigned_jobs():
    try:
        res = requests.get(f"{SCHEDULER_URL}/nodes/{NODE_ID}/jobs")
        print("response" , res.json())
        if res.status_code == 200:
            return res.json()
        else:
            print("[-] Failed to fetch jobs:", res.text)
            return []
    except Exception as e:
        print("[-] Error fetching jobs:", e)
        return []

# def execute_job(job):
#     print(f"[+] Executing job {job['id']}...")
#     time.sleep(5)  # simulate
#     result_hash = hashlib.sha256(f"{job['id']}-{time.time()}".encode()).hexdigest()
#     logs = f"Job {job['id']} executed successfully"
#     return result_hash, logs

def submit_result(job_id, result_hash, logs):
    global NODE_ID
    if NODE_ID is None:
        print("[-] Cannot submit result, NODE_ID unknown")
        return
    payload = {
        "node_id": NODE_ID,
        "result_hash": result_hash,
        "logs": logs
    }
    print(payload)
    try:
        res = requests.post(f"{SCHEDULER_URL}/nodes/{job_id}/result", json=payload)
        print("response" , res.status_code, res.text, res)
        if res.status_code == 200:
            print(f"[+] Result submitted for job {job_id}")
        else:
            print("[-] Failed to submit result:", res.text)
    except Exception as e:
        print("[-] Error submitting result:", e)

def main():
    while True:
        print("[*] Polling for assigned jobs...")   
        jobs = get_assigned_jobs()
        for job in jobs:
            print(f"[*] Found job: {job['jobId']} with status {job['status']}")
            if job["status"] == "pending" or job["status"] == "failed":
                print("[*] Attempting to assign self as provider...")
                assign_provider(job["jobId"], MY_ADDRESS)
                print(f"[*] Starting execution of job {job['jobId']}")
                result_hash, logs = execute_job(job)
                print(f"[*] Job {job['jobId']} completed with result hash {result_hash}")
                submit_result(job["jobId"], result_hash, logs)
                print(f"[*] Submitted result for job {job['jobId']}")
        time.sleep(POLL_INTERVAL)

if __name__ == "__main__":
    main()
