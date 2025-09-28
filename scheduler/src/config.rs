use dotenvy::dotenv;
use ethers::abi::Abi;
use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::sync::Arc;

pub type Client = SignerMiddleware<Provider<Http>, Wallet<SigningKey>>;
pub type YourContractError = ContractError<Client>;

pub struct AppConfig {
    pub port: u16,
    pub provider: Arc<Provider<Http>>,
    pub wallet_address: Address,
    pub contract: Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
    pub owner_contract: Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
}



impl AppConfig {
    pub async fn new() -> anyhow::Result<Self> {
        dotenv().ok();

        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(3000);

        let rpc_url = env::var("RPC_URL")
            .expect("Missing RPC_URL in .env");
        let private_key = env::var("PRIVATE_KEY")
            .expect("Missing PRIVATE_KEY in .env");
        let private_key_owner = env::var("PRIVATE_KEY_OWNER")
            .expect("Missing PRIVATE_KEY_OWNER in .env");
        let contract_address = env::var("CONTRACT_ADDRESS")
            .expect("Missing CONTRACT_ADDRESS in .env");

        let abi_path = "abi/JobRegistryABI.json";
        let abi_json = fs::read_to_string(abi_path)?;
        let artifact: serde_json::Value = serde_json::from_str(&abi_json)?;
        let abi_array = artifact["abi"].to_string();
        let abi: Abi = serde_json::from_str(&abi_array)?; 
        let provider = Provider::<Http>::try_from(rpc_url)?;

        let chain_id = provider.get_chainid().await?.as_u64();

        let wallet: LocalWallet = private_key.parse()?;
        let wallet = wallet.with_chain_id(chain_id);
        let wallet_mw = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

        let owner_wallet: LocalWallet = private_key_owner.parse()?;
        let owner_wallet = owner_wallet.with_chain_id(chain_id);
        let owner_wallet_mw = Arc::new(SignerMiddleware::new(provider.clone(), owner_wallet));

        let address: Address = contract_address.parse()?;
        let contract = Contract::new(address, abi.clone(), wallet_mw.clone());
        let owner_contract = Contract::new(address, abi, owner_wallet_mw.clone());

        Ok(Self {
            port,
            provider: Arc::new(provider),
            wallet_address: wallet_mw.address(),
            contract,
            owner_contract,
        })
    }
}
