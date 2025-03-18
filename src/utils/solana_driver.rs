use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    system_transaction,
    signature::{Keypair, Signature, Signer},
};
use std::{error::Error, str::FromStr};

const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

pub struct SolanaDriver {
    client: RpcClient,
    agent_keypair: Keypair,
}

impl SolanaDriver {
    pub fn new_solana_driver(rpc_url: &str, agent_keypair: Keypair) -> Self {
        Self {
            client: RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed()),
            agent_keypair,
        }
    }

    pub fn get_agent_balance(&self) -> Result<u64, Box<dyn Error>> {
        let balance = self.client.get_balance(&self.agent_keypair.pubkey())?;
        Ok(balance)
    }

    pub fn transfer_share_to_user(&self, user_address: &str, amount: u64) -> Result<Signature, Box<dyn Error>> {
        let to_pubkey = Pubkey::from_str(user_address)?;
        let recent_blockhash = self.client.get_latest_blockhash()?;

        let tx = system_transaction::transfer(&self.agent_keypair, &to_pubkey, amount, recent_blockhash);

        let signature = self.client.send_and_confirm_transaction(&tx)?;
        Ok(signature)
    }

    pub fn get_address(&self) -> Pubkey {
        self.agent_keypair.pubkey()
    }
}
