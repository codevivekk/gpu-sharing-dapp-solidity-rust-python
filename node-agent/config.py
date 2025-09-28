import os
from web3 import Web3
from dotenv import load_dotenv
import json

# Load .env
load_dotenv()

RPC_URL = os.getenv("RPC_URL")
PRIVATE_KEY = os.getenv("PRIVATE_KEY")
CONTRACT_ADDRESS = os.getenv("CONTRACT_ADDRESS")
ABI_PATH = "./abi/JobRegistryABI.json"

if not RPC_URL or not PRIVATE_KEY or not CONTRACT_ADDRESS:
    raise ValueError("Missing required environment variables in .env")

# Connect to provider
w3 = Web3(Web3.HTTPProvider(RPC_URL))
if not w3.is_connected:
    raise ConnectionError(f"Failed to connect to RPC URL: {RPC_URL}")

print(f"Connected to network: {w3.eth.chain_id}")


print(f"Connected to network: {w3.eth.chain_id}")

account = w3.eth.account.from_key(PRIVATE_KEY)
wallet_address = account.address
print(f"Wallet address: {wallet_address}")

# Load contract ABI
with open(ABI_PATH, "r") as f:
    contract_json = json.load(f)
    contract_abi = contract_json["abi"]

# Create contract instance
contract = w3.eth.contract(address=Web3.to_checksum_address(CONTRACT_ADDRESS), abi=contract_abi)


config = {
    "w3": w3,
    "contract": contract,
    "account": account,
    "wallet_address": wallet_address
}
