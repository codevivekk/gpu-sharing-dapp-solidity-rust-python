from execute_job import execute_job

# Simulate a frontend job
dummy_job = {
    "id": "job_test_1",
    "datasetCID": "https://ipfs.io/ipfs/QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u",
    "containerImage": "python:3.12-slim"
}

result_hash, logs = execute_job(dummy_job)

print("Result Hash:", result_hash)
print("Logs:", logs)
