use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub struct AgaveClient {
    pub client: RpcClient,
}

impl AgaveClient {
    pub fn new() -> Self {
        let rpc_url = "https://rpc.agave.io"; 
        Self { client: RpcClient::new(rpc_url.to_string()) }
    }

    pub fn get_balance(&self, address: &str) -> Result<u64, String> {
        let pubkey = Pubkey::from_str(address).map_err(|_| "Invalid address".to_string())?;
        self.client.get_balance(&pubkey).map_err(|e| e.to_string())
    }
}
