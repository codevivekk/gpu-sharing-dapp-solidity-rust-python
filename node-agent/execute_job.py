import subprocess
import hashlib
import os
import shutil
import tempfile
import requests

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
IPFS_PATH = os.path.join(BASE_DIR, "job_data")

def download_dataset(dataset_cid, job_folder):
    if dataset_cid.startswith("http"):
        try:
            local_file = os.path.join(job_folder, "dataset_file")
            print(f"[+] Downloading dataset from {dataset_cid}...")
            r = requests.get(dataset_cid)
            r.raise_for_status()
            with open(local_file, "wb") as f:
                f.write(r.content)
            print(f"[+] Dataset downloaded to {local_file}")
            return True, None
        except Exception as e:
            logs = f"Failed to download dataset from URL: {e}"
            print("[-]", logs)
            return False, logs
    else:
        # IPFS CLI is skipped entirely for now
        logs = "Skipping IPFS CLI because it's not installed"
        print("[!] IPFS CLI not available, skipping download")
        return False, logs


def run_docker_container(job_folder, container_image, job_id):
    print(f"[!] Skipping Docker execution for testing. Job folder: {job_folder}")
    # Just read the dataset file for testing
    dataset_file = os.path.join(job_folder, "dataset_file")
    with open(dataset_file, "r") as f:
        print(f"[+] Dataset content:\n{f.read()}")
    return True, "Docker skipped for testing"


def compute_result_hash(job_folder):
    sha256 = hashlib.sha256()
    for root, _, files in os.walk(job_folder):
        for file in sorted(files):
            file_path = os.path.join(root, file)
            with open(file_path, "rb") as f:
                for chunk in iter(lambda: f.read(4096), b""):
                    sha256.update(chunk)
    result_hash = sha256.hexdigest()
    print(f"[+] Result hash: {result_hash}")
    return result_hash

def execute_job(job):
    job_id = job['jobId']
    dataset_cid = job['dataset']
    container_image = job['containerCID']

    print(f"[+] Executing job {job_id}...")

    job_folder = os.path.join(IPFS_PATH, job_id)
    if os.path.exists(job_folder):
        shutil.rmtree(job_folder)
    os.makedirs(job_folder, exist_ok=True)

    success, logs = download_dataset(dataset_cid, job_folder)
    if not success:
        return None, logs

    success, logs = run_docker_container(job_folder, container_image, job_id)
    if not success:
        return None, logs

    result_hash = compute_result_hash(job_folder)
    result_hash = "0x" + result_hash  # Prefix with 0x for hex representation

    if os.path.exists(job_folder):
        shutil.rmtree(job_folder)
        print(f"[+] Cleaned up job folder: {job_folder}")

    return result_hash, logs
